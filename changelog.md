# 0.1.5
- Update Keywords to be valid for [Crates.io](crates.io)
- Update Docs for some functions
- Added Readme to the `lib.rs` file
- Add `get_bool` function to get config values as Booleans

## 0.1.4
 - Add Keywords, Categories and documentation to Cargo.toml
 - Allow for comments mid line
 - Remove Trailing / Leading spaces in Key and Value
 - Use more constants in the code
 - Ignore Sections for the time being
 - Add Benchmarks
 - Most recently defined config options will be used first
    - Ex: If you have a config file that defines the key hello twice the one lower in the file will be used
- Make `cfg.get(key)` return a `Option<String>` instead of a `String`