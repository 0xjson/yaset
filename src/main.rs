use clap::Parser;
use colored::*;
use std::path::PathBuf;
use yaset::{enumerate_subdomains};

mod banner;

#[derive(Parser, Debug)]
#[command(name = "yaset")]
#[command(version = "0.1.0")]
#[command(about = "Yet Another Subdomain Enumeration Tool", long_about = None)]
struct Args {
    #[arg(short = 'd', long = "domain", required = true)]
    domain: String,

    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    #[arg(short,long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    banner::show_banner();
    let args = Args::parse();

    match enumerate_subdomains(&args.domain).await {
        Ok(subdomains) => {
            let count = subdomains.len();

            if args.verbose{
                // print all subdomains in verbose mode
                let mut sorted: Vec<String> = subdomains.into_iter().collect();
                sorted.sort();


                let mut output: Box<dyn std::io::Write> = match args.output {
                    Some(path) => Box::new(std::fs::File::create(path).unwrap()),
                    None => Box::new(std::io::stdout()),
                };


                for subdomain in sorted {
                    writeln!(output, "{}", subdomain)?;
                }
            } else {
                // print just the count in non-verbose mode
                println!("{}", format!("subdomains: {}", count).green());
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "error".red(), e);
            std::process::exit(1)
        }
    }
    Ok(())
}
