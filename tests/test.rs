use simple_config_parser::config::Config;

#[test]
/// Test parsing a config string
fn test_config_from_string() {
    let mut cfg = Config::new(None);
    cfg.parse(
        "
    hello = world
    rust = is great
    test = \"TEST\"\
    ",
    )
    .ok()
    .unwrap();

    assert_eq!(cfg.get("hello").unwrap(), "world");
    assert_eq!(cfg.get("rust").unwrap(), "is great");
    assert_eq!(cfg.get("test").unwrap(), "\"TEST\"");
}

#[test]
/// Test loading and parsing a config file
fn test_config_from_file() {
    let mut cfg = Config::new(Some("config.cfg"));
    cfg.read().ok().unwrap();

    assert_eq!(cfg.get("hello").unwrap(), "World");
    assert_eq!(cfg.get("rust").unwrap(), "Is great");
    assert_eq!(cfg.get("test").unwrap(), "\"TEST\"");
}

#[test]
#[should_panic]
/// Test loading and parsing a config file with comments
fn test_config_from_string_with_comments() {
    let mut cfg = Config::new(None);
    cfg.parse(
        "
    hello = world
    #rust = is great
    ;test = \"TEST\"\
    ",
    )
    .ok()
    .unwrap();

    assert_eq!(cfg.get("hello").unwrap(), "world");

    // This should panic
    cfg.get("rust").unwrap();
    cfg.get("test").unwrap();
}

#[test]
/// Test case sensitivity of config keys
fn test_case_sensitivity() {
    let mut cfg = Config::new(None);
    cfg.parse(
        "
    HeLlO = world
    RuSt = is great
    TeSt = \"TEST\"\
    ",
    )
    .ok()
    .unwrap();

    assert_eq!(cfg.get("hello").unwrap(), "world");
    assert_eq!(cfg.get("RUST").unwrap(), "is great");
    assert_eq!(cfg.get("tEsT").unwrap(), "\"TEST\"");
}

#[test]
/// Test comments in a config file not at the beginning of a line
fn test_comments_in_file() {
    let mut cfg = Config::new(None);
    cfg.parse(
        "
    hello = world
    rust = is great# Comment
    test = \"TEST\"; Nother Comment\
    ",
    )
    .ok()
    .unwrap();

    assert_eq!(cfg.get("hello").unwrap(), "world");
    assert_eq!(cfg.get("RUST").unwrap(), "is great");
    assert_eq!(cfg.get("tEsT").unwrap(), "\"TEST\"");
}

#[test]
/// Test parseing will Ignore Sections
fn test_ignoring_sections() {
    let mut cfg = Config::new(None);
    cfg.parse(
        "
    [section]
    hello = world
    [section2]
    rust = is great
    [section3]
    test = \"TEST\"\
    ",
    )
    .ok()
    .unwrap();

    assert_eq!(cfg.get("hello").unwrap(), "world");
    assert_eq!(cfg.get("rust").unwrap(), "is great");
    assert_eq!(cfg.get("test").unwrap(), "\"TEST\"");
}

#[test]
/// The most recently defined key will be used
fn test_last_key_wins() {
    let mut cfg = Config::new(None);
    cfg.parse(
        "
    hello = people
    hello = world
    ",
    )
    .ok()
    .unwrap();

    assert_eq!(cfg.get("hello").unwrap(), "world");
}
