use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directories containing repos, eg "../my-repos".
    #[arg(short, long)]
    repo: Vec<String>,

    /// Type of behavior/output.
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Produce an mdbook of the specified recipe.
    Mdbook {
        /// The name of the recipe to create MD Book for, eg "my_widget".
        target: String,
    },
    /// Show parsed recipe, probably not very useful.
    Info {
        /// The name of the recipe to show info for, eg "my_widget".
        target: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut repos = tools::Repos::default();
    for repo_path in &args.repo {
        repos.add_repo(repo_path)?;
    }

    match &args.command {
        Commands::Mdbook { target } => {
            let build_plan = repos.compile(target)?;
            build_plan.make_mdbook()?;
        }
        Commands::Info { target } => {
            let recipe = repos.get_recipe(target)?;
            println!("{:#?}", recipe);
        }
    }

    Ok(())
}
