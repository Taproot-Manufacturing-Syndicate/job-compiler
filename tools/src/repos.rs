use std::io::Write;
use std::process::Command;
use std::str::FromStr;

use thiserror::Error;

use crate::recipe::Recipe;

use crate::recipe_id::RecipeId;
use crate::recipe_id::RecipeIdParseError;

use crate::repo::Repo;

#[derive(Default)]
pub struct Repos {
    repos: std::collections::HashMap<String, Repo>,
}

#[derive(Debug, Error, PartialEq)]
pub enum RecipeLookupError {
    #[error(transparent)]
    RecipeIdParseError(RecipeIdParseError),
    #[error("recipe not found: {0}")]
    RecipeNotFound(String),
    #[error("repo not found: {0}")]
    RepoNotFound(String),
}

impl From<RecipeIdParseError> for RecipeLookupError {
    fn from(val: RecipeIdParseError) -> Self {
        RecipeLookupError::RecipeIdParseError(val)
    }
}

#[derive(Debug, Error)]
pub enum AddRepoError {
    #[error("repo load error: {0}")]
    RepoLoadError(#[from] crate::repo::RepoLoadError),
    #[error("invalid repo name: {0}")]
    InvalidRepoName(String),
}

#[derive(Debug, thiserror::Error)]
pub enum RecipeCompileError {
    #[error(transparent)]
    RecipeLookupError(#[from] RecipeLookupError),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error("vitamin has no capital input: {0}")]
    VitaminLacksCapital(String),
    #[error("vitamin capital has wrong unit: {0}")]
    VitaminCapitalHasWrongUnit(String),
    #[error("input produces no output: {0}")]
    InputProducesNoOutput(String),
}

impl Repos {
    pub fn add_repo(&mut self, repo_path: &str) -> Result<(), AddRepoError> {
        let repo_pathbuf = std::path::PathBuf::from(repo_path);
        match repo_pathbuf.file_name() {
            Some(repo_name) => {
                let repo = Repo::new(repo_path)?;
                let repo_name = repo_name
                    .to_str()
                    .ok_or_else(|| AddRepoError::InvalidRepoName(repo_path.into()))?;
                self.repos.insert(repo_name.into(), repo);
                println!("added repo {repo_name}");
                Ok(())
            }
            None => {
                todo!();
            }
        }
    }

    // Recipe names are of the form "[repo:]recipe".
    // If repo is specified, look only in that repo.
    // If repo is not specified, search all repos.
    pub fn get_recipe(&self, recipe_name: &str) -> Result<&Recipe, RecipeLookupError> {
        let recipe_id = RecipeId::from_str(recipe_name)?;
        match recipe_id.repo {
            Some(repo) => {
                let repo = match self.repos.get(&repo) {
                    Some(repo) => repo,
                    None => return Err(RecipeLookupError::RepoNotFound(repo)),
                };
                repo.get_recipe(&recipe_id.recipe)
                    .ok_or_else(|| RecipeLookupError::RecipeNotFound(recipe_name.into()))
            }
            None => {
                for (_repo_name, repo) in self.repos.iter() {
                    if let Some(recipe) = repo.get_recipe(recipe_name) {
                        return Ok(recipe);
                    }
                }
                Err(RecipeLookupError::RecipeNotFound(recipe_name.into()))
            }
        }
    }

    pub fn compile(&self, target: &str) -> Result<(), RecipeCompileError> {
        let recipe = self.get_recipe(target)?;
        let puml_filename = format!("{target}.puml");
        let mut puml_file = std::fs::File::create(&puml_filename)?;
        writeln!(puml_file, "@startuml")?;
        writeln!(puml_file, "object {}", target)?;
        self.compile_inner(&mut puml_file, target, recipe, 4)?;
        writeln!(puml_file, "@enduml")?;
        Command::new("plantuml").arg("-v").arg(&puml_filename).output().expect("failed to run `plantuml`");
        Ok(())
    }
}

impl Repos {
    fn compile_inner(
        &self,
        puml_file: &mut std::fs::File,
        recipe_name: &str, // the name of the thing we're making
        recipe: &Recipe,   // the recipe for the thing we're making
        indent: usize,
    ) -> Result<(), RecipeCompileError> {
        for (input_name, input_info) in recipe.inputs.iter() {
            let input_recipe = self.get_recipe(input_name)?;

            // A Recipe whose only input is Capital is a Vitamin.
            let cost = match input_recipe.is_vitamin() {
                true => {
                    let amount_needed = input_info.quantity;

                    // FIXME: for now Vitamins must have exactly one Input, and it must be Capital.
                    assert_eq!(input_recipe.inputs.len(), 1);
                    let input_capital = input_recipe.inputs.get("capital").ok_or_else(|| {
                        RecipeCompileError::VitaminLacksCapital(input_name.into())
                    })?;
                    if input_capital.quantity.unit != Some(crate::recipe::Unit::USDollar) {
                        return Err(RecipeCompileError::VitaminCapitalHasWrongUnit(
                            input_name.into(),
                        ));
                    }
                    let cost = input_capital.quantity.amount;

                    let outputs = match &input_recipe.outputs {
                        None => {
                            return Err(RecipeCompileError::InputProducesNoOutput(
                                input_name.into(),
                            ));
                        }
                        Some(outputs) => outputs,
                    };

                    // FIXME: For now Vitamins must produce exactly one output.
                    assert_eq!(outputs.len(), 1);

                    let (_output_name, output_info) = outputs.iter().next().unwrap();
                    let output_quantity = output_info.quantity;

                    // compute the "unit cost" of this input
                    let cost_each = cost / output_quantity.amount;
                    let total_cost = amount_needed.amount * cost_each;

                    // FIXME: Try to coerce the units here - if we buy
                    // hose in rolls measured in feet, but we use lengths
                    // of hose measured in meters, that should be ok.
                    if let Some(output_unit) = output_info.quantity.unit {
                        Some(format!(
                            "${:.6} / {:?}\ntotal: ${:.6}",
                            cost_each, output_unit, total_cost
                        ))
                    } else {
                        Some(format!("${:.6} each\ntotal: ${:.6}", cost_each, total_cost))
                    }
                }
                false => None,
            };

            let (input_object_name, input_object_declaration) = match input_recipe.is_vitamin() {
                true => {
                    // "Leaf" nodes get unique boxes.
                    let uuid = uuid::Uuid::new_v4();
                    let input_object_name = format!("uuid_{}", uuid.simple());
                    (
                        input_object_name.clone(),
                        format!("\"{}\" as {}", input_name, input_object_name),
                    )
                }
                false => (input_name.clone(), input_name.clone()),
            };

            writeln!(puml_file, "object {}", input_object_declaration)?;
            match &input_recipe.action {
                crate::recipe::Action::process(process) => {
                    writeln!(puml_file, "{} : {:?}", input_object_name, process)?;
                }
                crate::recipe::Action::print => {
                    writeln!(puml_file, "{} : print", input_object_name)?;
                }
                crate::recipe::Action::purchase(_purchase) => {
                    writeln!(puml_file, "{} : buy", input_object_name)?;
                }
            }

            if let Some(cost_str) = cost {
                writeln!(puml_file, "{} : {:?}", input_object_name, cost_str)?;
            }

            write!(puml_file, "{} <|-- {}", recipe_name, input_object_name)?;
            if input_info.quantity.unit.is_some() || input_info.quantity.amount != 1.0 {
                write!(puml_file, " : quantity={:?}", input_info.quantity)?;
            }
            writeln!(puml_file)?;

            if !input_recipe.is_vitamin() {
                self.compile_inner(puml_file, input_name, input_recipe, indent + 4)?;
            }
        }

        Ok(())
    }
}

mod test {
    #[test]
    fn empty_repos() {
        let empty_repos = crate::repos::Repos::default();
        assert_eq!(
            empty_repos.get_recipe("nonexistent".into()),
            Err(crate::repos::RecipeLookupError::RecipeNotFound(
                "nonexistent".into()
            ))
        );
    }

    #[test]
    fn load_two_repos() -> Result<(), crate::repos::AddRepoError> {
        let mut repos = crate::repos::Repos::default();
        repos.add_repo("../modular-recipes/repos/fasteners")?;
        repos.add_repo("../modular-recipes/recipes/peristaltic-pump")?;
        assert_eq!(repos.repos.iter().count(), 2);
        println!("{:?}", repos.repos.keys());
        let _fasteners = repos.repos.get("fasteners").unwrap();
        let _peristaltic_pump = repos.repos.get("peristaltic-pump").unwrap();
        Ok(())
    }

    #[test]
    fn get_recipe() {
        let mut repos = crate::repos::Repos::default();
        repos
            .add_repo("../modular-recipes/repos/fasteners")
            .unwrap();
        repos
            .add_repo("../modular-recipes/recipes/peristaltic-pump")
            .unwrap();
        repos
            .add_repo("../modular-recipes/repos/minimal_hydroponics_setup")
            .unwrap();
        let _r = repos.get_recipe("peristaltic_pump").unwrap();
        let _r = repos.get_recipe("m4_nuts").unwrap();
        let _r = repos.get_recipe("fasteners:m4_nuts").unwrap();
        let _r = repos.get_recipe("barb_fitting_1_2_inch").unwrap();
    }
}
