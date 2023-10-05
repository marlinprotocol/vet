use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// url to query
    #[clap(short, long, value_parser)]
    url: String,
}

fn main() {
    let cli = Cli::parse();

    println!("url: {}", cli.url);
}
