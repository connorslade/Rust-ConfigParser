# Rust-ConfigParser [![CI](https://github.com/Basicprogrammer10/Rust-ConfigParser/actions/workflows/main.yml/badge.svg)](https://github.com/Basicprogrammer10/Rust-ConfigParser/actions/workflows/main.yml) ![Crates.io](https://img.shields.io/crates/d/simple_config_parser) ![Lines of code](https://img.shields.io/tokei/lines/github/Basicprogrammer10/Rust-ConfigParser)
⚙ Very simple config parsing lib for rust!

## 💠 Install

Just add the following to your `Cargo.toml`:
```toml
[dependencies]
simple_config_parser = "0.1.3"
```

## 📀 Quick Start

This config parser is made for use with a simplified version of an ini file. There are no sections and currently no Escape character support.
```ini
; This is a comment
# This is also a comment
hello = World
rust = Is great
test = "TEST"
```

## 💥Examples

Import Module.
```rust
// Import Lib
use simple_config_parser::config::Config;
```

Create a new config.
```rust
// Create a new config with no file
let mut cfg = Config::new(None);

// Create a new config with a file
let mut cfg2 = Config::new(Some("config.cfg"));
```

Read a config file and parse it.
```rust
// Create a new config with a file
let mut cfg2 = Config::new(Some("config.cfg"));

// Read / parse config file
cfg.read().ok().expect("Error reading the config file");
```

Load config from a string.
```rust
// Create a new config with no file
let mut cfg = Config::new(None);

// Parse config from string
cfg.parse("hello = World\nrust = Is great\ntest = \"TEST\"").ok().expect("Error parsing the config file");
```

Get a value from a config.
```rust
// Create a new config with no file
let mut cfg = Config::new(None);

// Create a new config with a file
let mut cfg2 = Config::new(Some("config.cfg"));

// Get a value from the config (As a string)
println!("Hello, {}", cfg.get("hello"));
```