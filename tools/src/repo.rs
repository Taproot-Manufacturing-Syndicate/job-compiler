use crate::recipe::Recipe;

#[derive(Debug)]
pub struct Repo {
    path: String,
    recipes: std::collections::HashMap<String, Recipe>,
}

impl Repo {
    pub fn new(path: &str) -> Self {
        let mut repo = Self {
            path: std::string::String::from(path),
            recipes: std::collections::HashMap::<String, Recipe>::new(),
        };
        repo.add_dir(path).unwrap();
        repo
    }

    pub fn get_recipe(self: &Self, recipe_name: &str) -> Option<&Recipe> {
        self.recipes.get(recipe_name)
    }

    pub fn compile(self: &Self, target: &str) -> anyhow::Result<()> {
        let recipe = self.get_recipe(target);

        // FIXME: Accumulate `inputs` from our callees, pass up to our
        // caller via the Result?
        //
        // Or have a "compile context" argument with the "cost" in it,
        // and update that as we go?
        //
        // Maybe in the context we should also construct a parallel
        // build process?

        // let inputs = std::collections::HashMap::<String, usize>::new();
        // let price_per_unit = std::collections::HashMap::<String, f32>::new();

        match recipe {
            None => {
                return Err(anyhow::Error::msg(format!("recipe for {target} not found")));
            }
            Some(recipe) => {
                println!("building {target:#?}");
                println!("inputs:");
                self.compile_inner(recipe, 4)
            }
        }
    }
}

impl Repo {
    fn add_dir(self: &mut Self, path: &str) -> anyhow::Result<()> {
        // println!("reading Recipes from {path}");
        let dir_entries = std::fs::read_dir(path).unwrap();
        for dir_entry in dir_entries {
            let dir_entry = dir_entry.unwrap();
            let file_type = dir_entry.file_type().unwrap();
            // println!("trying {:?} ({:?})", dir_entry, file_type);
            if file_type.is_file() {
                let path = dir_entry.path();
                match self.add_file(&path) {
                    Ok(()) => {
                        // println!("added {:?}", dir_entry);
                    }
                    Err(e) => {
                        println!("failed to read recipe from {:?}: {:?}", path, e);
                    }
                }
            } else if file_type.is_dir() {
                let _ = self.add_dir(dir_entry.path().to_str().unwrap());
            }
        }
        Ok(())
    }

    fn add_file(self: &mut Self, path: &std::path::PathBuf) -> anyhow::Result<()> {
        // println!("reading Recipe from {:?}", path);
        if let Some(recipe_name) = path.file_stem() {
            let key = recipe_name.to_string_lossy().into_owned();
            let value = crate::Recipe::from_file(path)?;
            self.recipes.insert(key, value);
        }
        Ok(())
    }

    fn compile_inner(self: &Self, recipe: &Recipe, indent: usize) -> anyhow::Result<()> {
        for (input_name, input_info) in recipe.inputs.iter() {
            for _ in 0..indent {
                print!(" ");
            }

            if input_name == "capital" {
                // The build process begins in capitalism :-(

                let input_unit = match &input_info.quantity.unit {
                    None => {
                        return Err(anyhow::Error::msg(format!(
                            "expected quantity unit USDollar on capital input"
                        )))
                    }
                    Some(unit) if *unit != crate::recipe::Unit::USDollar => {
                        return Err(anyhow::Error::msg(format!(
                            "expected quantity unit USDollar on capital input"
                        )))
                    }
                    Some(unit) => unit,
                };

                let cost = input_info.quantity.amount;

                let outputs = match &recipe.outputs {
                    None => return Err(anyhow::Error::msg(format!("no outputs!"))),
                    Some(outputs) => outputs,
                };

                if outputs.len() == 1 {
                    for (_output_name, output_info) in outputs.iter() {
                        let cost_each = cost / output_info.quantity.amount;
                        if let Some(output_unit) = output_info.quantity.unit {
                            println!(
                                "capital: {:.3} {:?} / {:?}  :-(",
                                cost_each, input_unit, output_unit
                            );
                        } else {
                            println!("capital: {:.3} {:?} each :-(", cost_each, input_unit);
                        }
                    }
                } else {
                    return Err(anyhow::Error::msg(format!("no output!")));
                }
                continue;
            }
            println!("{input_name:?} ({:?})", input_info.quantity);
            match self.get_recipe(input_name) {
                None => {
                    return Err(anyhow::Error::msg(format!(
                        "recipe for {input_name} not found"
                    )));
                }
                Some(input_recipe) => {
                    self.compile_inner(input_recipe, indent + 4)?;
                }
            }
        }

        match &recipe.action {
            crate::recipe::Action::process(s) => {
                if let Some(tools) = &recipe.dependencies.tools {
                    for _ in 0..indent {
                        print!(" ");
                    }
                    println!("tools: {:?}", tools);
                }
                for _ in 0..indent {
                    print!(" ");
                }
                println!("action: {s}");
            }
            _ => (),
        }

        Ok(())
    }
}
