// #[derive(Debug, serde::Deserialize)]
// enum Quantity {
//     Count(usize),
//     Amount(String),
// }

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize)]
pub enum Unit {
    USDollar,
    Meter,
    Gram,
}

/// `Quantity` measures the amount of a resource (Input or Output).
#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct Quantity {
    pub amount: f32,
    pub unit: Option<Unit>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Input {
    // Quantity defaults to "amount=1" if omitted.
    pub quantity: Option<Quantity>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Output {
    // Quantity defaults to "amount=1" if omitted.
    pub quantity: Option<Quantity>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Operator {
    pub skills: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Dependencies {
    pub tools: Option<Vec<String>>,
    pub operator: Option<Operator>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Purchase {
    pub vendor: Vec<String>,
    pub documentation: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
pub enum Action {
    process(String),
    print,
    purchase(Purchase),
}

#[derive(Debug, serde::Deserialize)]
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

impl Input {
    pub fn get_quantity(&self) -> Quantity {
        match &self.quantity {
            None => Quantity {
                amount: 1.0,
                unit: None,
            },
            Some(q) => q.clone(),
        }
    }
}

impl Output {
    pub fn get_quantity(&self) -> Quantity {
        match &self.quantity {
            None => Quantity {
                amount: 1.0,
                unit: None,
            },
            Some(q) => q.clone(),
        }
    }
}

impl Recipe {
    pub fn from_file(file: &std::path::PathBuf) -> anyhow::Result<Self> {
        let recipe_contents = std::fs::read_to_string(file)?;
        let recipe: Recipe = toml::from_str(&recipe_contents)?;
        // let r = recipe.validate_recipe();
        Ok(recipe)
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
