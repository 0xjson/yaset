use colored::*;

pub fn show_banner() {
    println!(
        r#"
    {}

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
        "YESET".bright_cyan().bold(),
        "Yet Another Subdomain Enumeration Tool".bright_white(),
        env!("CARGO_PKG_VERSION").bright_green(),
        "by js0nn".bright_magenta()
    );
    println!("{}", "~".repeat(50).dimmed());
}
