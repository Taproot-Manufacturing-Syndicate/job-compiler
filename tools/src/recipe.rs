use crate::quantity::*;

#[derive(Debug, thiserror::Error)]
pub enum RecipeLoadError {
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    TomlDeserializeError(#[from] toml::de::Error),
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum VitaminError {
    #[error("not a vitamin")]
    NotAVitamin,
    #[error("vitamin has bogus inputs")]
    InputsError,
    #[error("vitamon has no output")]
    OutputError,
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
    pub image: Option<String>,
    pub comment: Option<String>,
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
pub struct PrintedPart {
    pub model: String,
    pub profile: String,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Action {
    process(String),
    print(PrintedPart),
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

    /// The recipe file.
    ///
    /// FIXME: This should not be an Option, but we can't parse it out
    /// of the file.
    pub path: Option<std::path::PathBuf>,
}

impl Recipe {
    pub fn from_file(file: &std::path::PathBuf) -> Result<Self, RecipeLoadError> {
        let recipe_contents = std::fs::read_to_string(file)?;
        let mut recipe: Recipe = toml::from_str(&recipe_contents)?;
        if recipe.outputs.is_none() {
            if let Some(recipe_name) = file.file_stem() {
                let mut outputs = std::collections::HashMap::<String, Output>::new();
                let key = recipe_name.to_string_lossy().into_owned();
                let value = Output {
                    quantity: Quantity::default(),
                    image: None,
                    comment: None,
                };
                outputs.insert(key, value);
                recipe.outputs = Some(outputs);
            }
        }
        recipe.path = Some(file.clone());
        recipe.validate_recipe()?;
        Ok(recipe)
    }

    /// Compute the cost (of quantity 1).  Currently only work on vitamins.
    pub fn unit_cost(&self) -> Result<f32, VitaminError> {
        if !self.is_vitamin() {
            return Err(VitaminError::NotAVitamin);
        }

        // Vitamins must have exactly one Input, and it must be Capital.
        if self.inputs.len() != 1 {
            return Err(VitaminError::InputsError);
        }

        let capital = match self.inputs.get("capital") {
            Some(capital) => capital,
            None => return Err(VitaminError::InputsError),
        };

        if capital.quantity.unit != Some(crate::quantity::Unit::USDollar) {
            return Err(VitaminError::InputsError);
        }

        let total_cost = capital.quantity.amount;

        let outputs = match &self.outputs {
            Some(outputs) => outputs,
            None => return Err(VitaminError::OutputError),
        };

        // FIXME: For now Vitamins must produce exactly one output.
        if outputs.len() != 1 {
            return Err(VitaminError::OutputError);
        }

        let (_output_name, output_info) = outputs.iter().next().unwrap();
        let output_quantity = output_info.quantity;

        // compute the "unit cost" of this input
        return Ok(total_cost / output_quantity.amount);
    }

    // A "Vitamin" is a recipe whose only input is "capital".
    pub fn is_vitamin(&self) -> bool {
        if self.inputs.len() != 1 {
            return false;
        }
        if let Some(input_name) = self.inputs.keys().next() {
            if input_name == "capital" {
                return true;
            }
        }
        false
    }

    // A "print" is a recipe whose Action is "print".
    pub fn is_print(&self) -> bool {
        match self.action {
            Action::print(_) => true,
            _ => false,
        }
    }
}

impl Recipe {
    fn validate_recipe(&self) -> Result<(), RecipeLoadError> {
        // if recipe.inputs.len() == 0 {
        //     Err("recipe has no inputs!");
        // }
        Ok(())
    }
}

mod test {
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
            let recipe = super::Recipe::from_file(&recipe_path).unwrap();
            let result = recipe.is_vitamin();
            assert_eq!(result, *is_leaf);
        }
    }
}
