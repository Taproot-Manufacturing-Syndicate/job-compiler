// #[derive(Debug, serde::Deserialize)]
// enum Quantity {
//     Count(usize),
//     Amount(String),
// }

#[derive(Debug, serde::Deserialize)]
pub struct Input {
    /// Each input has either a `quantity` (for discrete things, like
    /// apples or bearings) or an `amount` (for continuous things,
    /// like rope or flour).
    ///
    /// FIXME: Is there a good way to handle the "units" of the
    /// quantity/amount? Maybe the units could default to "count", but
    /// be optionally overridden by the recipe to things like "meters"
    /// (of tubing) or "grams" (of chromic acid).  Then this struct
    /// would be `quantity: usize, units: Option<Unit>`.
    quantity: Option<usize>,
    amount: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Operator {
    skills: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Dependencies {
    tools: Vec<String>,
    operator: Operator,
}

#[derive(Debug, serde::Deserialize)]
pub struct Action {
    process: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Recipe {
    /// `[inputs]` is a Table where each key the the name (unique id)
    /// of a recipe in the repo, and the value is an Input object.
    inputs: std::collections::HashMap<String, Input>,

    dependencies: Dependencies,

    action: Action,

    /// If a recipe has no `[outputs]`, we assume it produces 1x of the
    /// thing identified by the name of the recipe.
    ///
    /// FIXME: Or is that always the case, and we should have no outputs
    /// section?  None of the recipes we've been doodling around with
    /// have anything like byproducts or waste streams...
    outputs: Option<Vec<String>>,
}

impl Recipe {
    pub fn from_file(file: &std::path::PathBuf) -> anyhow::Result<Self> {
        let recipe_contents = std::fs::read_to_string(file)?;
        let recipe: Recipe = toml::from_str(&recipe_contents)?;
        // let r = recipe.validate_recipe();
        Ok(recipe)
    }

    fn validate_recipe(self: &Self) -> anyhow::Result<()> {
        // if recipe.inputs.len() == 0 {
        //     Err("recipe has no inputs!");
        // }
        Ok(())
    }
}
