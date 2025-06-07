use colored::*;

pub fn show_banner() {
    println!("{}", "~".repeat(50).dimmed());
    println!(
        r#"

    ▄██   ▄      ▄████████    ▄████████    ▄████████     ███     
    ███   ██▄   ███    ███   ███    ███   ███    ███ ▀█████████▄ 
    ███▄▄▄███   ███    ███   ███    █▀    ███    █▀     ▀███▀▀██ 
    ▀▀▀▀▀▀███   ███    ███   ███         ▄███▄▄▄         ███   ▀ 
    ▄██   ███ ▀███████████ ▀███████████ ▀▀███▀▀▀         ███     
    ███   ███   ███    ███          ███   ███    █▄      ███     
    ███   ███   ███    ███    ▄█    ███   ███    ███     ███     
     ▀█████▀    ███    █▀   ▄████████▀    ██████████    ▄████▀   
    {} v{} - {}
    "#,
        "Yet Another Subdomain Enumeration Tool".bright_cyan(),
        env!("CARGO_PKG_VERSION").bright_green(),
        "by js0nn".bright_magenta()
    );
    println!("{}", "~".repeat(50).dimmed());
}
