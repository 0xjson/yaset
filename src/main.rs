use clap::Parser;
use colored::*;
use std::path::PathBuf;
use yaset::{enumerate_subdomains, modules::asnmap};
// use std::env;

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
                println!("{} {}",
                    "Subdomains Found:".green(),
                    count.to_string().bright_white(),
                );
                println!("{}","~".repeat(50).dimmed());

            }
        }
        Err(e) => {
            eprintln!("{}: {}", "error".red(), e);
            std::process::exit(1)
        }
    }

    let api_key = std::env::var("CHAOS_API_KEY").unwrap_or_default(); 
    println!("{}", "ASN Information:".bright_blue().bold());

    if !api_key.is_empty() {
        match asnmap::enumerate(&args.domain, &api_key).await {
            Ok(entries) if !entries.is_empty() => {
                for entry in entries {
                    let ip_blocks = asnmap::calculate_ip_blocks(&entry);
                    println!(
                        "{} {} - {} - {}",
                        "ASN:".bright_green(),
                        entry.asn.to_string().bright_white(),
                        entry.country.bright_yellow(),
                        entry.org.bright_cyan()
                    );

                    for block in ip_blocks {
                        println!("   {}", block.bright_white());
                    }
                }
                println!("{}", "~".repeat(50).dimmed());
            }
            Ok(_) => {
                println!("  No ASN information found");
            }
            Err(e) => {
                println!("  Error: {}", e.to_string().red());
            }
        }   
    } else {
        println!("  ASN lookup requires CHAOS_API_KEY");
    }
    println!();
    Ok(())
}
