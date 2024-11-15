mod download;
mod export;
mod test;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "mataneko1366",
    version = "0.0.1",
    about = "atcoder-judger is a simple atcoder judge tool.",
    long_about = None
)]
struct Args {
    /// Option to download testcase
    #[arg(short, long)]
    download: Option<String>,

    /// Option to test testcase
    #[arg(short, long)]
    test: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.download.is_some() && args.test.is_some() {
        eprintln!("Error!: Download and test can't coexistence!");
    } else if args.download.is_none() && args.test.is_none() {
        eprintln!("Error!: Please input args!");
    } else if args.download.is_some() {
        download::download(args.download.clone().unwrap());
    } else if args.test.is_some() {
        test::test(args.test.clone().unwrap());
    }
}
