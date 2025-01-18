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

    let repo = Repo::new(&args.repo);
    println!("{:#?}", repo);

    let recipe: Option<&Recipe> = repo.get_recipe(&args.target);
    println!("{recipe:#?}");

    Ok(())
}
