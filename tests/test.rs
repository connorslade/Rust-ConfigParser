use simple_config_parser::Config;

#[test]
/// Test parsing a config string
fn test_config_from_string() {
    let cfg = Config::new()
        .text("hello = world\nrust = is great\ntest = \"TEST\"")
        .unwrap();

    assert_eq!(cfg.get_str("hello").unwrap(), "world");
    assert_eq!(cfg.get_str("rust").unwrap(), "is great");
    assert_eq!(cfg.get_str("test").unwrap(), "\"TEST\"");
}

#[test]
/// Test loading and parsing a config file
fn test_config_from_file() {
    let cfg = Config::new().file("config.cfg").unwrap();

    assert_eq!(cfg.get_str("hello").unwrap(), "World");
    assert_eq!(cfg.get_str("rust").unwrap(), "Is great");
    assert_eq!(cfg.get_str("test").unwrap(), "\"TEST\"");
}

#[test]
#[should_panic]
/// Test loading and parsing a config file with comments
fn test_config_from_string_with_comments() {
    let cfg = Config::new()
        .text("hello = world\n#rust = is great\n;test = \"TEST\"")
        .unwrap();

    assert_eq!(cfg.get_str("hello").unwrap(), "world");

    // This should panic
    cfg.get_str("rust").unwrap();
    cfg.get_str("test").unwrap();
}

#[test]
/// Test case sensitivity of config keys
fn test_case_sensitivity() {
    let cfg = Config::new()
        .text("HeLlO = world\nRuSt = is great\nTeSt = \"TEST\"")
        .unwrap();

    assert_eq!(cfg.get_str("hello").unwrap(), "world");
    assert_eq!(cfg.get_str("RUST").unwrap(), "is great");
    assert_eq!(cfg.get_str("tEsT").unwrap(), "\"TEST\"");
}

#[test]
/// Test comments in a config file not at the beginning of a line
fn test_comments_in_file() {
    let cfg = Config::new()
        .text("hello = world\nrust = is great # Comment\ntest = \"TEST\"; Nother Comment")
        .unwrap();

    assert_eq!(cfg.get_str("hello").unwrap(), "world");
    assert_eq!(cfg.get_str("RUST").unwrap(), "is great");
    assert_eq!(cfg.get_str("tEsT").unwrap(), "\"TEST\"");
}

#[test]
/// Test parseing will Ignore Sections
fn test_ignoring_sections() {
    let cfg = Config::new()
        .text("[section]\nhello = world\n[section2]\nrust = is great\n[section3]\ntest = \"TEST\"")
        .unwrap();

    assert_eq!(cfg.get_str("hello").unwrap(), "world");
    assert_eq!(cfg.get_str("rust").unwrap(), "is great");
    assert_eq!(cfg.get_str("test").unwrap(), "\"TEST\"");
}

#[test]
/// The most recently defined key will be used
fn test_last_key_wins() {
    let cfg = Config::new().text("hello = people\nhello = world").unwrap();

    assert_eq!(cfg.get_str("hello").unwrap(), "world");
}

#[test]
/// Test getting value as a bool
fn test_bool_value() {
    let cfg = Config::new().text("test = true\ntset = false").unwrap();

    assert_eq!(cfg.get::<bool>("test").unwrap(), true);
    assert_eq!(cfg.get::<bool>("tset").unwrap(), false);
}

#[test]
/// Test getting value as a integer
fn test_int_value() {
    let cfg = Config::new().text("test = 1\ntset = 100").unwrap();

    assert_eq!(cfg.get::<i32>("test").unwrap(), 1);
    assert_eq!(cfg.get::<i32>("tset").unwrap(), 100);
}

#[test]
/// Test getting value as a float
fn test_float_value() {
    let cfg = Config::new()
        .text("test = 1.123\npi = 3.14159 ; And so on")
        .unwrap();

    assert_eq!(cfg.get::<f32>("test").unwrap(), 1.123);
    assert_eq!(cfg.get::<f32>("pi").unwrap(), 3.14159);
}

#[test]
/// Test the priority of loading data more than one time
fn test_load_priority() {
    let cfg = Config::new()
        .text("a = 1\nb = 2")
        .unwrap()
        .text("a = 2\nb = 4")
        .unwrap();

    assert_eq!(cfg.get_str("a").unwrap(), "2");
    assert_eq!(cfg.get_str("b").unwrap(), "4");
}
