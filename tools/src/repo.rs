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
        match recipe {
            None => {
                return Err(anyhow::Error::msg(format!("recipe for {target} not found")));
            }
            Some(recipe) => {
                println!("{recipe:#?}");
                println!("inputs:");
                for (key, val) in recipe.inputs.iter() {
                    println!("{key:?} {val:?}");
                    if key == "capital" {
                        println!("aquire capital: {:?}", val.amount.to_owned());
                    } else {
                        self.compile(key)?;
                    }
                }
            }
        }
        Ok(())
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
}
