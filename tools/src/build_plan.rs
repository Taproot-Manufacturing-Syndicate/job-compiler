// This is a representation of the recipes/steps that result in a
// desired Output.  It's a path from starting points/vitamins to the
// desired Output.

use std::io::Write;
use std::process::Command;

use crate::quantity::*;

#[derive(Debug, thiserror::Error)]
pub enum MdbookError {
    #[error("error from mdbook: {0:?}")]
    MdbookProcessError(std::process::Output),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error("error with build_plan data structure: {0:?}")]
    BuildPlanError(String),
    #[error(transparent)]
    RecipeLookupError(#[from] crate::repos::RecipeLookupError),
    #[error(transparent)]
    VitaminError(#[from] crate::recipe::VitaminError),
}

// FIXME: this isn't the correct abstraction for Item
#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub quantity: Quantity,
}

#[derive(Debug)]
pub struct BuildPlan<'a> {
    /// The name of the Item this build plan builds.
    pub name: &'a str,

    /// Bill of materials needed to build this Item.
    /// Keys are the names of Items.
    /// Values are Items, which are just the name and a Quantity.
    pub bom: std::collections::HashMap<String, Item>,

    pub repos: &'a crate::repos::Repos,
}

impl<'a> BuildPlan<'a> {
    pub fn new(target: &'a str, repos: &'a crate::repos::Repos) -> Self {
        BuildPlan {
            name: target,
            bom: std::collections::HashMap::<String, Item>::new(),
            repos,
        }
    }

    pub fn make_mdbook(&self) -> Result<(), MdbookError> {
        let mdbook_dir = format!("{}.mdbook", self.name);
        self.write_mdbook(&mdbook_dir)?;

        let output = Command::new("mdbook")
            .arg("build")
            .arg(&mdbook_dir)
            .output()
            .expect("failed to run `mdbook build`");
        if !output.status.success() {
            println!("failed to build mdbook");
            return Err(MdbookError::MdbookProcessError(output));
        }

        Ok(())
    }
}

impl<'a> BuildPlan<'a> {
    fn write_mdbook(&self, mdbook_dir: &str) -> Result<(), MdbookError> {
        let output = Command::new("mdbook")
            .arg("init")
            .arg("--title")
            .arg(self.name)
            .arg("--ignore")
            .arg("git")
            .arg(&mdbook_dir)
            .output()
            .expect("failed to run `mdbook init`");
        if !output.status.success() {
            println!("failed to create mdbook");
            return Err(MdbookError::MdbookProcessError(output));
        }

        // # Summary
        //
        // [Intro](./intro.md)
        // - [Chapter 1](./chapter_1.md)
        // - [Chapter 2](./chapter_2.md)
        //     - [Chapter 2, Section 1](./chapter2_1.md)
        //     - [Chapter 2, Section 2](./chapter2_2.md)

        let summary_md_filename = format!("{mdbook_dir}/src/SUMMARY.md");
        let mut summary_md_file = std::fs::File::create(&summary_md_filename)?;
        writeln!(summary_md_file, "# Summary")?;
        writeln!(summary_md_file, "")?;

        self.write_mdbook_overview(&mdbook_dir, &mut summary_md_file)?;
        self.write_mdbook_chapters(self.name, &mdbook_dir, &mut summary_md_file)?;

        Ok(())
    }

    fn write_mdbook_overview(
        &self,
        mdbook_dir: &str,
        summary_md_file: &mut std::fs::File,
    ) -> Result<(), MdbookError> {
        let overview_dirname = format!("{mdbook_dir}/src/overview");
        std::fs::create_dir(&overview_dirname)?;

        let overview_md_filename = format!("{overview_dirname}/overview.md");
        let mut overview_md_file = std::fs::File::create(&overview_md_filename)?;

        writeln!(overview_md_file, "# Overview")?;
        writeln!(overview_md_file, "We're making {}.", self.name)?;

        writeln!(summary_md_file, "- [Overview](overview/overview.md)")?;

        self.write_mdbook_bom(&overview_dirname, summary_md_file)?;
        Ok(())
    }

    fn write_mdbook_bom(
        &self,
        overview_dirname: &str,
        summary_md_file: &mut std::fs::File,
    ) -> Result<(), MdbookError> {
        let bom_md_filename = format!("{overview_dirname}/bom.md");
        let mut bom_md_file = std::fs::File::create(&bom_md_filename)?;

        writeln!(bom_md_file, "# Bill of materials")?;
        writeln!(bom_md_file, "")?;

        let mut names: Vec<&String> = self.bom.keys().collect();
        names.sort();
        for name in names {
            let item = self
                .bom
                .get(name)
                .ok_or(MdbookError::BuildPlanError(format!(
                    "item {name} not found"
                )))?;
            let recipe = self.repos.get_recipe(name)?;
            let unit_cost = recipe.unit_cost()?;
            let total_cost = unit_cost * item.quantity.amount;
            writeln!(bom_md_file, "* {name}")?;
            writeln!(bom_md_file, "    * quantity {:?}", item.quantity)?;
            writeln!(
                bom_md_file,
                "    * cost {:.2} ({:.2} each)",
                total_cost, unit_cost
            )?;
            writeln!(bom_md_file, "    * vendors:")?;
            match &recipe.action {
                crate::recipe::Action::purchase(purchase) => {
                    for vendor in &purchase.vendor {
                        writeln!(bom_md_file, "        * [{}]({})", vendor, vendor)?;
                    }
                }
                _ => {
                    panic!(
                        "bom item {} has unexpected action {:?}, expected `purchase`",
                        name, &recipe.action
                    );
                }
            }
        }

        writeln!(
            summary_md_file,
            "    - [Bill of Materials](overview/bom.md)"
        )?;

        Ok(())
    }

    // Do a depth-first traversal of the DAG.
    fn write_mdbook_chapters(
        &self,
        recipe_name: &str,
        mdbook_dir: &str,
        summary_md_file: &mut std::fs::File,
    ) -> Result<(), MdbookError> {
        let recipe = self.repos.get_recipe(recipe_name)?;
        if recipe.is_vitamin() {
            return Ok(());
        }

        let mut input_names: Vec<&String> = recipe.inputs.keys().collect();
        input_names.sort();
        for input_name in input_names {
            self.write_mdbook_chapters(input_name, mdbook_dir, summary_md_file)?;
        }

        // for (input_name, input_info) in recipe.inputs.iter() {
        //     let input_recipe = self.repos.get_recipe(input_name)?;
        //     self.write_mdbook_chapters(input_recipe, mdbook_dir, summary_md_file)?;
        // }

        // Write the chapter on this recipe.
        let chapter_md_filename = format!("{mdbook_dir}/src/{recipe_name}.md");
        let mut chapter_md_file = std::fs::File::create(&chapter_md_filename)?;

        writeln!(chapter_md_file, "# {recipe_name}")?;
        writeln!(chapter_md_file, "")?;

        if let Some(tools) = &recipe.dependencies.tools {
            writeln!(chapter_md_file, "## Tools")?;
            let mut tools: Vec<&String> = tools.iter().collect();
            tools.sort();
            for tool in tools {
                writeln!(chapter_md_file, "* {tool}")?;
            }
        }

        if let Some(operator) = &recipe.dependencies.operator {
            let mut skills = operator.skills.clone();
            if skills.len() > 0 {
                writeln!(chapter_md_file, "## Operator Skills")?;
                skills.sort();
                for skill in skills {
                    writeln!(chapter_md_file, "* {skill}")?;
                }
            }
        }

        writeln!(chapter_md_file, "## Inputs")?;
        let mut input_names: Vec<&String> = recipe.inputs.keys().collect();
        input_names.sort();
        for input_name in input_names {
            let item = recipe
                .inputs
                .get(input_name)
                .ok_or(MdbookError::BuildPlanError(format!(
                    "item {input_name} not found"
                )))?;
            let input_recipe = self.repos.get_recipe(input_name)?;
            let how_we_got_it = {
                if input_recipe.is_vitamin() {
                    String::from("vitamin")
                } else if input_recipe.is_print() {
                    String::from("printed")
                } else {
                    String::from("assembled")
                }
            };

            writeln!(
                chapter_md_file,
                "* {input_name} (quantity {:?}, {})",
                item.quantity, how_we_got_it
            )?;
        }

        writeln!(chapter_md_file, "## Action")?;
        match &recipe.action {
            crate::recipe::Action::process(process) => {
                writeln!(chapter_md_file, "{}", process)?;
            }
            _ => {
                panic!(
                    "item {} has unexpected action {:?}, expected `process`",
                    recipe_name, &recipe.action
                );
            }
        }

        // Add this chapter to the book.
        writeln!(summary_md_file, "- [{recipe_name}](./{recipe_name}.md)")?;

        Ok(())
    }
}
