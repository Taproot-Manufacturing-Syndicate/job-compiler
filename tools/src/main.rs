use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    target: String,
}

// #[derive(Debug, serde::Deserialize)]
// enum Quantity {
//     Count(usize),
//     Amount(String),
// }

#[derive(Debug, serde::Deserialize)]
struct Input {
    quantity: Option<usize>,
    amount: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct Operator {
    skills: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct Dependencies {
    tools: Vec<String>,
    operator: Operator,
}

#[derive(Debug, serde::Deserialize)]
struct Action {
    process: String,
}

#[derive(Debug, serde::Deserialize)]
struct Recipe {
    inputs: std::collections::HashMap<String, Input>,
    dependencies: Dependencies,
    action: Action,
    outputs: Option<Vec<String>>,
}

fn validate_recipe(recipe: &Recipe) -> anyhow::Result<()> {
    // if recipe.inputs.len() == 0 {
    //     Err("recipe has no inputs!");
    // }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let target_str = std::fs::read_to_string(args.target)?;
    let recipe: Recipe = toml::from_str(&target_str)?;
    println!("{recipe:#?}");
    let r = validate_recipe(&recipe);
    println!("valid?  {r:#?}");
    Ok(())
}
