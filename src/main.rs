use clap::Parser;
use colored::*;
use std::path::PathBuf;
use yaset::{enumerate_subdomains, modules::asnmap, modules::hprobe};
// use std::env;
use std::io::Write;

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

    #[arg(short = 'c', long = "concurrency", default_value_t = 50)]
    concurrency: usize,

    #[arg(short= 'u', long = "user-agent")]
    user_agent: Option<String>,

    #[arg(long = "timeout", default_value_t = 10)]
    timeout: u64,

    #[arg(long = "delay", default_value_t = 1)]
    delay: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    banner::show_banner();
    let args = Args::parse();

    match enumerate_subdomains(&args.domain).await {
        Ok(subdomains) => {
            let count = subdomains.len();
            let subdomains: Vec<String> = subdomains.into_iter().collect();

            if args.verbose {
                let mut sorted = subdomains.clone();
                sorted.sort();

                match args.output {
                    Some(path) => {
                        // For files, we can write everything at once after processing
                        let mut file =  std::fs::File::create(path)?;

                        //Write all subdomain first
                        for subdomain in &sorted {
                            writeln!(file, "{}", subdomain)?;
                        }
                        // writeln!(file, "{} {}", "Subdomains Found:".green(), count.to_string().bright_white())?;

                        let config = hprobe::ProbeConfig {
                            concurrency: args.concurrency,
                            timeout: args.timeout,
                            delay: args.delay,
                            user_agent: args.user_agent,
                        };

                        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
                        let _alive_count = hprobe::probe_hosts_with_progress(sorted, config, tx).await?;

                        // Collect alive Hosts
                        let alive_hosts: Vec<String> = {
                        let mut v = Vec::new();
                           while let Some(host) = rx.recv().await {
                                v.push(host);
                           }
                           v 
                        };
                        
                        // Write alive hosts to file!
                        for host in &alive_hosts {
                            writeln!(file, "{}", host.bright_green())?; 
                        }
                        writeln!(file, "{} {}", "Subdomains Found:".green(), count.to_string().bright_white())?;
                        writeln!(file, "{} {}", "Alive Hosts:".bright_green(), alive_hosts.len())?;

                    }
                    None => {
                        // For stdout, we can print in real-time
                        // println!("{} {}", "Subdomain Found:".green(), count.to_string().bright_white());
                        for subdomain in &sorted {
                            println!("  {}", subdomain);
                        }

                        let config = hprobe::ProbeConfig {
                            concurrency: args.concurrency,
                            timeout: args.timeout,
                            delay: args.delay,
                            user_agent: args.user_agent,
                        };

                        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
                        let print_task = tokio::spawn(async move {
                            let mut alive_count = 0;
                            while let Some(host) = rx.recv().await {
                                println!("  {}", host.bright_white());
                                alive_count += 1;
                            }
                            alive_count
                        });

                        let alive_count = hprobe::probe_hosts_with_progress(sorted, config, tx).await?;
                        let _ = print_task.await?;
                        println!("{} {}", "Subdomain Found:".green(), count.to_string().bright_white());
                        println!("{} {}", "Alive Hosts:".bright_green(), alive_count);
                    }
                }
            

                // let mut output: Box<dyn std::io::Write> = match args.output {
                //     Some(path) => Box::new(std::fs::File::create(path)?),
                //     None => Box::new(std::io::stdout()),
                // };
                //
                // for subdomain in &sorted {
                //     writeln!(output, "  {}", subdomain)?;
                // }
                //
                // writeln!(output, "{} {}", "Subdomains Found:".green(), count.to_string().bright_white())?;
                // // Configure probing
                // let config = hprobe::ProbeConfig {
                //     concurrency: args.concurrency,
                //     timeout: args.timeout,
                //     delay: args.delay,
                //     user_agent: args.user_agent,
                // };
                //
            //     let alive = hprobe::probe_hosts(sorted, config).await?;
            //     writeln!(output, "{} {}", "Alive Hosts:".bright_green(), alive.len())?;
            //     for host in &alive {
            //         writeln!(output, "  {}", host.bright_white())?;
            //     }
            // } else {
            //     // Non-verbose mode
            //     println!("\n{} {}",
            //         "Subdomains Found:".green(),
            //         count.to_string().bright_white()
            //     );
            //
            //     let config = hprobe::ProbeConfig::default();
            //     let alive = hprobe::probe_hosts(subdomains, config).await?;
            //     println!("\n{} {}", "Alive Hosts:".bright_green(), alive.len());
            //     println!("{}", "~".repeat(50).dimmed());
            //     let (tx ,rx ) = tokio::sync::mpsc::channel::<String>(100);
            //
            //     let print_task = tokio::spawn(async move {
            //         let mut alive_hosts = Vec::new();
            //         while let Some(host) = rx.recv().await {
            //             writeln!(output, "  {}", host.bright_white()).unwrap();
            //             alive_hosts.push(host);
            //         }
            //         // writeln!(output, "{} {}", "Alive Hosts:".bright_green(), alive_hosts.len()).unwrap();
            //         alive_hosts
            //     });
            //
            //     let alive_count = hprobe::probe_hosts_with_progress(sorted, config, tx).await?;
            //     let _alive = print_task.await?;
            //     writeln!(output, "{} {}", "Alive Hosts:".bright_green(), alive_count)?;
            } else {
                // Non-verbose mode
                println!("\n{} {}",
                    "Subdomains Found:".green(),
                    count.to_string().bright_white()
                );

                let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
                let config = hprobe::ProbeConfig::default();
                // let alive_count = hprobe::probe_hosts_with_progress(subdomains, config, tx).await?;

                // In Non-verbose , we still need to consume the channel
                let count_task = tokio::spawn(async move {
                    let mut count = 0;
                    while let Some(_) = rx.recv().await {
                        count += 1;
                    }
                    count
                });

                let _ = hprobe::probe_hosts_with_progress(subdomains, config, tx).await?;
                let alive_count = count_task.await?;

                println!("\n{} {}", "Alive Hosts:".bright_green(), alive_count);
                println!("{}", "~".repeat(50).dimmed());                                                                                    
            }
        }
        Err(e) => {
            println!("{} {}", "error".red(), e);
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
    Ok(())
}
