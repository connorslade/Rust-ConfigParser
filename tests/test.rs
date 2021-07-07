use simple_config_parser::config::Config;

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
    let mut cfg = Config::new(Some("config.cfg"));
    cfg.read().ok().unwrap();

    assert_eq!(cfg.get("hello"), "World");
    assert_eq!(cfg.get("rust"), "Is great");
    assert_eq!(cfg.get("test"), "\"TEST\"");
}

#[test]
/// Test loading and parsing a config file with comments
fn test_config_from_string_with_comments() {
    let mut cfg = Config::new(None);
    cfg.parse("hello = world\n#rust = is great\n;test = \"TEST\"").ok().unwrap();

    assert_eq!(cfg.get("hello"), "world");
    assert_eq!(cfg.get("rust"), "");
    assert_eq!(cfg.get("test"), "");
}

#[test]
/// Test case sensitivity of config keys
fn test_case_sensitivity() {
    let mut cfg = Config::new(None);
    cfg.parse("HeLlO = world\nRuSt = is great\nTeSt = \"TEST\"").ok().unwrap();

    assert_eq!(cfg.get("hello"), "world");
    assert_eq!(cfg.get("RUST"), "is great");
    assert_eq!(cfg.get("tEsT"), "\"TEST\"");
}