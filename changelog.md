# Changelog
## 1.0.0
- Cleanup Syntax of Loading / Reading Config
- Remove all get_(bool, int, float)
- Add get_str for getting raw config value
- add get<T> for gatting a config value as any type (that impls FromStr)
- Update Exmaples
- Cleanup Code

## 0.1.6
- Add docs for internal functions
- Update some formatting in the readme
- Make the parsed config data available to the user

## 0.1.5
- Update Keywords to be valid for [Crates.io](crates.io)
- Update Docs for some functions
- Added Readme to the `lib.rs` file
- Add `get_bool` function to get config values as Booleans
- Add `get_int` function to get config values as Integers
- Add `get_float` function to get config values as Floats

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
