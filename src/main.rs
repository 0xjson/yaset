use clap::Parser;
use std::path::PathBuf;
use yaset::{enumerate_subdomains};


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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let subdomains = enumerate_subdomains(&args.domain)?;

    if args.verbose {
        eprintln!("Found {} unique subdomains (after filtering wildcards)", subdomains.len());
    }

    let mut output: Box<dyn std::io::Write> = match args.output {
        Some(path) => Box::new(std::fs::File::create(path)?),
        None => Box::new(std::io::stdout()),
    };

    let mut sorted: Vec<String> = subdomains.into_iter().collect();
    sorted.sort();

    for subdomain in sorted {
        writeln!(output, "{}", subdomain)?;
    }

    Ok(())
}
