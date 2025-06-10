# YAET - Yet Another Enumeration Tool

![Rust](https://img.shields.io/badge/Rust-1.70+-informational?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)
![Build](https://img.shields.io/github/actions/workflow/status/yourusername/yaset/rust.yml)

A high-performance subdomain enumeration tool written in Rust, featuring:
- Multi-threaded DNS enumeration
- Concurrent HTTP/HTTPS probing
- ASN information lookup
- Real-time results output

## Features

🚀 **Fast Enumeration**
- Multi-threaded DNS resolution
- Configurable concurrency (default: 50 threads)

🔍 **Smart Probing**
- Simultaneous HTTP/HTTPS checks
- Customizable timeouts and delays
- Random user-agent rotation

📊 **Flexible Output**
- Verbose mode with detailed results
- File output support
- Colorized terminal output

🌐 **ASN Integration**
- Optional Chaos API integration
- IP block calculations
- Organization/country information

## Installation

### From Source
```bash
git clone https://github.com/yourusername/yaset.git
cd yaset
cargo build --release
