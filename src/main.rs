mod dblp;
mod hal;

use clap::Parser;

trait BibtexServer {
    fn fetch(&self, query: &str) -> String;
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    queries: Vec<String>,
    #[clap(short, long)]
    dblp: bool,
    #[clap(short, long)]
    hal: bool,
}

fn main() {
    let dblp_api = dblp::API {};
    let hal_api = hal::API {};
    let args: Cli = Cli::parse();
    for query in &args.queries {
        if args.dblp {
            println!("{}", dblp_api.fetch(query.as_str()));
        }
        if args.hal {
            println!("{}", hal_api.fetch(query.as_str()));
        }
    }
}