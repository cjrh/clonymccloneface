use std::path::PathBuf;

mod gh;

#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short, long)]
    username: String,
    #[structopt(short, long)]
    token: String,
    // Add an optional comma-separated list of repo names to clone.
    // If the list is empty, clone all repos.
    #[structopt(short, long)]
    repos: Option<String>,
    // #[structopt(short, long, default_value = ".")]
    // path: String,
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        gh::get_repos_list(&args.username, &args.token, &args.path, args.repos)
            .await
            .expect("Problem running.")
    });
    Ok(())
}
