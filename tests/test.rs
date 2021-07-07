use std::fs::File;
use std::io::prelude::*;

#[test]
/// Test parsing a config string
fn test_config_from_string() {
    let mut cfg = Config::new(None);
    cfg.parse("hello = world\nrust = is great\ntest = \"TEST\"")
        .ok()
        .unwrap();

    assert_eq!(cfg.get("hello"), "world");
    assert_eq!(cfg.get("rust"), "is great");
    assert_eq!(cfg.get("test"), "\"TEST\"");
}

#[test]
/// Test loading and parsing a config file
fn test_config_from_file() {
    // Create a conf file
    let mut file = File::create("config.cfg").unwrap();
    file.write_all(b"hello = world\n\rrust = is great\n\rtest = \"TEST\"")
        .unwrap();

    let mut cfg = Config::new(Some("config.cfg"));
    cfg.read().ok().unwrap();

    assert_eq!(cfg.get("hello"), "world");
    assert_eq!(cfg.get("rust"), "is great");
    assert_eq!(cfg.get("test"), "\"TEST\"");
}
