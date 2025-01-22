use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the recipe to build.
    target: String,

    /// Directories containing repos.
    #[arg(short, long)]
    // repo: String,
    repo: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // let repo = tools::Repo::new(&args.repo).unwrap();
    // repo.compile(&args.target)?;
    let mut repos = tools::Repos::default();
    for repo_path in &args.repo {
        repos.add_repo(repo_path)?;
    }
    let build_plan = repos.compile(&args.target)?;
    println!("{build_plan:#?}");
    Ok(())
}
