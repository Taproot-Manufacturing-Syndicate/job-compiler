use std::io::Write;
use std::process::Command;

use crate::recipe::Recipe;

#[derive(Debug)]
pub struct Repo {
    /// Canonical absolute path in the local filesystem where this repo
    /// is available.
    path: std::path::PathBuf,

    /// Parsed versions of all Recipes in this repo.
    recipes: std::collections::HashMap<String, Recipe>,
}

#[derive(Debug, thiserror::Error)]
pub enum RepoLoadError {
    #[error("io error: {0}")]
    StdIoError(#[from] std::io::Error),
}

impl Repo {
    pub fn new(path: &str) -> Result<Self, RepoLoadError> {
        let mut repo = Self {
            path: std::fs::canonicalize(std::path::PathBuf::from(path))?,
            recipes: std::collections::HashMap::<String, Recipe>::new(),
        };
        repo.add_dir(path)?;
        Ok(repo)
    }

    pub fn get_recipe(&self, recipe_name: &str) -> Option<&Recipe> {
        self.recipes.get(recipe_name)
    }

    pub fn get_path(&self) -> &std::path::PathBuf {
        &self.path
    }

    pub fn compile(&self, target: &str) -> anyhow::Result<()> {
        let recipe = self.get_recipe(target);

        match recipe {
            None => Err(anyhow::Error::msg(format!("recipe for {target} not found"))),
            Some(recipe) => {
                let puml_filename = format!("{target}.puml");
                let mut puml_file = std::fs::File::create(&puml_filename)?;
                writeln!(puml_file, "@startuml")?;
                writeln!(puml_file, "object {}", target)?;
                self.compile_inner(&mut puml_file, target, recipe, 4)?;
                writeln!(puml_file, "@enduml")?;
                let output = Command::new("plantuml")
                    .arg("-v")
                    .arg(&puml_filename)
                    .output()?;
                if !output.status.success() {
                    println!("{:#?}", output);
                }
                Ok(())
            }
        }
    }
}

impl Repo {
    fn add_dir(&mut self, path: &str) -> Result<(), RepoLoadError> {
        // println!("reading Recipes from {path}");
        let dir_entries = std::fs::read_dir(path)?;
        for dir_entry in dir_entries {
            let dir_entry = dir_entry.unwrap();
            let file_type = dir_entry.file_type().unwrap();
            // println!("trying {:?} ({:?})", dir_entry, file_type);
            if file_type.is_file() {
                let path = dir_entry.path();
                match path.extension() {
                    Some(s) if s.eq("toml") => {
                        match self.add_file(&path) {
                            Ok(()) => {
                                // println!("added {:?}", dir_entry);
                            }
                            Err(e) => {
                                println!("failed to read recipe from {:?}: {:?}", path, e);
                            }
                        }
                    }
                    _ => (), // skip unknown file extensions
                }
            } else if file_type.is_dir() {
                let _ = self.add_dir(dir_entry.path().to_str().unwrap());
            }
        }
        Ok(())
    }

    fn add_file(&mut self, path: &std::path::PathBuf) -> anyhow::Result<()> {
        // println!("reading Recipe from {:?}", path);
        if let Some(recipe_name) = path.file_stem() {
            let key = recipe_name.to_string_lossy().into_owned();
            let value = match Recipe::from_file(path) {
                Ok(recipe) => recipe,
                Err(e) => {
                    return Err(anyhow::Error::msg(format!(
                        "failed to read recipe {}: {:?}",
                        key, e
                    )));
                }
            };
            self.recipes.insert(key, value);
        }
        Ok(())
    }

    fn compile_inner(
        &self,
        puml_file: &mut std::fs::File,
        recipe_name: &str, // the name of the thing we're making
        recipe: &Recipe,   // the recipe for the thing we're making
        indent: usize,
    ) -> anyhow::Result<()> {
        for (input_name, input_info) in recipe.inputs.iter() {
            let input_recipe = match self.get_recipe(input_name) {
                Some(recipe) => recipe,
                None => {
                    return Err(anyhow::Error::msg(format!(
                        "failed to get recipe '{}'",
                        input_name
                    )));
                }
            };

            // A Recipe whose only input is Capital is a Vitamin.
            let cost = match input_recipe.is_vitamin() {
                true => {
                    let amount_needed = input_info.quantity;

                    // FIXME: for now Vitamins must have exactly one Input, and it must be Capital.
                    assert_eq!(input_recipe.inputs.len(), 1);
                    let input_capital =
                        input_recipe
                            .inputs
                            .get("capital")
                            .ok_or(anyhow::Error::msg(format!(
                                "can't find input capital for {input_name}"
                            )))?;
                    if input_capital.quantity.unit != Some(crate::quantity::Unit::USDollar) {
                        return Err(anyhow::Error::msg(format!(
                            "{} input capital does not have units USDollar",
                            input_name
                        )));
                    }
                    let cost = input_capital.quantity.amount;

                    let outputs = match &input_recipe.outputs {
                        None => {
                            return Err(anyhow::Error::msg(format!(
                                "{} has no outputs!",
                                input_name
                            )))
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
                crate::recipe::Action::print(printed_part) => {
                    writeln!(puml_file, "{} : {:?}", input_object_name, printed_part)?;
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
