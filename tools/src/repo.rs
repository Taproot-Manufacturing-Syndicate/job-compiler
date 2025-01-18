use crate::recipe::Recipe;
use std::fmt;

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
        for (key, val) in recipe.inputs.iter() {
            for _ in 0..indent {
                print!(" ");
            }

            if key == "capital" {
                // The build process begins in capitalism :-(

                if let Some(amount) = &val.amount {
                    // I'd like to divide the amount of capital by the
                    // quantity of the thing purchased.
                    //
                    // We can probably assume the `recipe.outputs` Option
                    // is Some, but inside the Some is a container with
                    // potentially any number of output things, which
                    // makes it hard to account.
                    //
                    // For now i'll just handle the common case specially.

                    if let Some(a) = amount.split_whitespace().next() {
                        let cost = a.parse::<f32>().unwrap();
                        if let Some(outputs) = &recipe.outputs {
                            if outputs.len() == 1 {
                                for (_k, v) in outputs.iter() {
                                    if let Some(quantity) = &v.quantity {
                                        println!(
                                            "capital: {cost}/{quantity} = {:.2} each :-(",
                                            cost / *quantity as f32
                                        );
                                    } else if let Some(amount) = &v.amount {
                                        println!("capital: {cost}/{amount:?} :-(",);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    panic!("no amount of capital?");
                }
                continue;
            }
            println!("{key:?} {val:?}");
            match self.get_recipe(key) {
                None => {
                    return Err(anyhow::Error::msg(format!("recipe for {key} not found")));
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
