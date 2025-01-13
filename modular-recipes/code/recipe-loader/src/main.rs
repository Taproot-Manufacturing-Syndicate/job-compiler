use clap::Parser;
use recipe::TomlRecipe;

use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    /// the recipe you'd like to make (needs to be in the repo)
    recipe: String,

    #[arg(long)]
    /// the repository of your recipes
    repo: String,
}

pub struct LoadedEnvironment {
    recipes: Vec<TomlRecipe>,
}

impl Default for LoadedEnvironment {
    fn default() -> Self {
        Self {
            recipes: Vec::new(),
        }
    }
}

impl LoadedEnvironment {
    pub fn new(repo: &Path) -> Self {
        let paths = fs::read_dir(repo).unwrap();
        let mut env = LoadedEnvironment::default();
        for path in paths {
            let path = path.expect("Failed to unwrap path");
            env.recipes.push(TomlRecipe::new(&path.path()))
        }
        env
    }

    pub fn print(&self) {
        for recipe in &self.recipes {
            recipe.print();
            println!();
        }
    }
}

pub fn main() {
    let cli = Cli::parse();
    let repo = Path::new(&cli.repo);
    let env = LoadedEnvironment::new(repo);
    env.print();
    // let recipe = TomlRecipe::default();
    // recipe.print();

    println!("{}", repo.display());
}
