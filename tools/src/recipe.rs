#[derive(Debug, thiserror::Error)]
pub enum RecipeLoadError {
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    TomlDeserializeError(#[from] toml::de::Error),
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize)]
pub enum Unit {
    Foot,
    Gram,
    Liter,
    Meter,
    USDollar,
}

/// `Quantity` measures the amount of a resource (Input or Output).
#[derive(Clone, Copy, PartialEq, serde::Deserialize)]
pub struct Quantity {
    pub amount: f32,
    pub unit: Option<Unit>,
}

impl std::default::Default for Quantity {
    fn default() -> Self {
        Self {
            amount: 1.0,
            unit: None,
        }
    }
}

impl std::fmt::Debug for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.unit {
            None => write!(f, "{:#}", self.amount),
            Some(unit) => write!(f, "{:#} {:?}", self.amount, unit),
        }
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Input {
    // Quantity defaults to "amount=1" if omitted.
    #[serde(default)]
    pub quantity: Quantity,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Output {
    // Quantity defaults to "amount=1" if omitted.
    #[serde(default)]
    pub quantity: Quantity,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Operator {
    pub skills: Vec<String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Dependencies {
    pub tools: Option<Vec<String>>,
    pub operator: Option<Operator>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Purchase {
    pub vendor: Vec<String>,
    pub documentation: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Action {
    process(String),
    print,
    purchase(Purchase),
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Recipe {
    /// `[inputs]` is a Table where each key the the name (unique id)
    /// of a recipe in the repo, and the value is an Input object.
    pub inputs: std::collections::HashMap<String, Input>,

    pub dependencies: Dependencies,

    pub action: Action,

    /// If a recipe has no `[outputs]`, we assume it produces 1x of the
    /// thing identified by the name of the recipe.
    ///
    /// FIXME: Or is that always the case, and we should have no outputs
    /// section?  None of the recipes we've been doodling around with
    /// have anything like byproducts or waste streams...
    pub outputs: Option<std::collections::HashMap<String, Output>>,
}

impl Recipe {
    pub fn from_file(file: &std::path::PathBuf) -> Result<Self, RecipeLoadError> {
        let recipe_contents = std::fs::read_to_string(file)?;
        let mut recipe: Recipe = toml::from_str(&recipe_contents)?;
        if recipe.outputs == None {
            if let Some(recipe_name) = file.file_stem() {
                let mut outputs = std::collections::HashMap::<String, Output>::new();
                let key = recipe_name.to_string_lossy().into_owned();
                let value = Output {
                    quantity: Quantity::default(),
                };
                outputs.insert(key, value);
                recipe.outputs = Some(outputs);
            }
        }
        // let r = recipe.validate_recipe();
        Ok(recipe)
    }

    // A "Vitamin" is a recipe whose only input is "capital".
    pub fn is_vitamin(&self) -> bool {
        if self.inputs.len() != 1 {
            return false;
        }
        if let Some(input_name) = self.inputs.keys().into_iter().next() {
            if input_name == "capital" {
                return true;
            }
        }
        return false;
    }
}

impl Recipe {
    fn validate_recipe(self: &Self) -> anyhow::Result<()> {
        // if recipe.inputs.len() == 0 {
        //     Err("recipe has no inputs!");
        // }
        Ok(())
    }
}

mod test {
    use super::*;

    #[test]
    fn is_vitamin() {
        let recipes = vec![
            (
                "../modular-recipes/recipes/peristaltic-pump/peristaltic_pump.toml",
                false,
            ),
            (
                "../modular-recipes/recipes/peristaltic-pump/print/bearing_hub.toml",
                false,
            ),
            ("../modular-recipes/repos/fasteners/m4_nuts.toml", true),
            ("../modular-recipes/repos/fasteners/filament.toml", true),
        ];

        for (recipe_filename, is_leaf) in recipes.iter() {
            let recipe_path = std::path::PathBuf::from(recipe_filename);
            let recipe = Recipe::from_file(&recipe_path).unwrap();
            let result = recipe.is_vitamin();
            assert_eq!(result, *is_leaf);
        }
    }
}
