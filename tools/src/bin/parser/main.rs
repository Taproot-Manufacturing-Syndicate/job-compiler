use clap::Parser;

use tools::Recipe;
use tools::Repo;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // The name of the recipe to build.
    target: String,

    /// Directory containing the repo of all repositories.
    #[arg(short, long)]
    repo: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let repo_contents = std::fs::read_dir(args.repo).unwrap();

    let target_str = std::fs::read_to_string(args.target)?;
    let recipe: Recipe = toml::from_str(&target_str)?;
    println!("{recipe:#?}");
    let r = tools::recipe::validate_recipe(&recipe);
    println!("valid?  {r:#?}");
    Ok(())
}
