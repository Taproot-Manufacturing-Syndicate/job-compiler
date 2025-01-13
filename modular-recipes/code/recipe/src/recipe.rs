use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use std::collections::BTreeMap;

#[derive(Hash, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Input {}

#[derive(Hash, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Output {}

#[derive(Hash, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Dependency {}

#[derive(Hash, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Config {}

// Recipe which can get read out of a toml file
#[derive(Deserialize, Serialize, Debug)]
pub struct TomlRecipe {
    name: String,
    inputs: BTreeMap<String, Input>,
    outputs: BTreeMap<String, Output>,
    dependencies: BTreeMap<String, Dependency>,
    config: Config,
}

impl Default for TomlRecipe {
    fn default() -> Self {
        Self {
            name: String::new(),
            inputs: BTreeMap::new(),
            outputs: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            config: Config {},
        }
    }
}

impl TomlRecipe {
    pub fn new(config_file: &Path) -> Self {
        let file: String = fs::read_to_string(&config_file).expect("Could not read input file");
        let recipe: TomlRecipe = toml::from_str(&file).expect("Could not parse TOML file");
        recipe
    }
    pub fn print(&self) {
        println!("{} ---------------", self.name);
        println!(" INPUTS");
        for (key, _) in &self.inputs {
            println!(" * {}", key);
        }
        println!();
        println!(" OUTPUTS");
        for (key, _) in &self.outputs {
            println!(" * {}", key);
        }
        println!();
        println!(" DEPENDENCIES");
        for (key, _) in &self.dependencies {
            println!(" * {}", key);
        }
    }
}
