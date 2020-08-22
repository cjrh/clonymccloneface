mod gh;

#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short, long)]
    username: String,
    #[structopt(short, long)]
    token: String,
}

#[paw::main]
fn main(args: Args) {
    gh::get_repos_list(&args.username, &args.token);
}
