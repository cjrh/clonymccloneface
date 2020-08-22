#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short = "t", long = "token")]
    token: String,
}

#[paw::main]
fn main(args: Args) {
    let token = args.token;
    println!("{}", &token);
}
