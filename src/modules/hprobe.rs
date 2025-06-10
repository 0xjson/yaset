use anyhow::Result;
use reqwest::{ Client, redirect::Policy};
use std::{sync::Arc ,time::Duration};
use tokio::sync::mpsc;
use std::sync::atomic::{Ordering, AtomicUsize};
// use tokio::time;
use futures::{stream, StreamExt};
use rand::Rng;
use rand::prelude::IndexedRandom;
// use colored::*;
// use rand::seq::SliceRandom;

pub struct ProbeConfig {
    pub concurrency: usize,
    pub timeout: u64,
    pub delay: u64,
    pub user_agent: Option<String>,
}

impl Default for ProbeConfig {
    fn default() -> Self {
        Self {
            concurrency: 50,
            timeout: 10,
            delay: 1,
            user_agent: None,
        }
    }
}

const USER_AGENTS: &[&str] = &["Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",];

fn get_random_user_agent() -> &'static str {
    let mut rng = rand::rng();
    // USER_AGENTS[rng.random_range(0..USER_AGENTS.len())]
    USER_AGENTS.choose(&mut rng).unwrap_or(&USER_AGENTS[0])
}

pub async fn probe_hosts_with_progress(hosts: Vec<String>, config: ProbeConfig, tx: mpsc::Sender::<String>) -> Result<usize> {
    // let (_tx, _rx) = mpsc::channel::<String>(config.concurrency);
    let client_builder = Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .redirect(Policy::limited(10));

    let client_builder = match config.user_agent {
        Some(ua) => client_builder.user_agent(ua),
        None => client_builder.user_agent(get_random_user_agent()),
    };

    let client = client_builder.build()?;
    let alive_count = Arc::new(AtomicUsize::new(0));
    // let hosts_len = hosts.len();
    // let (tx, mut rx) = mpsc::channel::<String>(config.concurrency);

    // let print_task = tokio::spawn(async move {
    //     let mut alive = Vec::with_capacity(hosts_len);
    //     while let Some(url) = rx.recv().await {
    //         // println!("\n{} {}", "Alive Hosts".bright_green(), url.len());
    //         // println!("  {}", url);
    //         alive.push(url);
    //     }
    //     alive
    // });

    stream::iter(hosts)
        .for_each_concurrent(config.concurrency, |host| {
            let client = client.clone();
            let tx = tx.clone();
            let alive_count = alive_count.clone();

            async move {
                if host.starts_with("http://") || host.starts_with("https://") {
                    if let Ok(resp) = client.get(&host).send().await {
                        if resp.status().is_success() {
                            alive_count.fetch_add(1, Ordering::Relaxed);
                            let _ = tx.send(host).await;
                        }
                    }
                    return;
                }

                let http_url = format!("http://{}", host);
                let https_url = format!("https://{}", host);

                let http_fut = client.get(&http_url).send();
                let https_fut = client.get(&https_url).send();

                match tokio::join!(http_fut, https_fut) {
                    (Ok(http_res), Ok(https_res)) => {
                        if http_res.status().is_success() {
                            alive_count.fetch_add(1, Ordering::Relaxed);
                            let _ = tx.send(http_url).await;
                        }                                                 
                        if https_res.status().is_success() {
                            alive_count.fetch_add(1, Ordering::Relaxed);
                            let _ = tx.send(https_url).await;
                        }                                                 
                    }
                    (Ok(http_res), _) => {
                        if http_res.status().is_success() {
                            alive_count.fetch_add(1, Ordering::Relaxed);
                            let _ = tx.send(https_url).await;
                        }                         
                    }
                    (_, Ok(https_res)) => {
                        if https_res.status().is_success() {
                                alive_count.fetch_add(1, Ordering::Relaxed);
                                let _ = tx.send(https_url).await;
                        }
                    }
                    _ => {}
                }
            }
        })
        .await;

    // drop(tx);

    // let mut alive = Vec::with_capacity(hosts_len);
    // while let Some(url) = rx.recv().await {
    //     alive.push(url);
    //
    // }
    //
    // Ok(alive)

    // Ok(print_task.await?)
    Ok(alive_count.load(Ordering::Relaxed))
}
