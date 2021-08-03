//! This module contains the things needed to load and parse ini like configuration files
use std::fs;

/// Define valid comment chars.
static COMMENT_CHARS: [&str; 2] = ["#", ";"];

/// Config Struct
pub struct Config {
    pub file: String,
    pub data: Vec<[String; 2]>,
}

/// Some errors that can be thrown by this module
pub enum ConfigError {
    /// Error reading the file from disk
    /// Could have been caused by the file not existing or being inaccessible.
    FileReadError,
    /// File path has not been defined
    /// You need to define the path to the config file before using this function.
    /// Or just use `cfg.parse("<STRING>");` instead.
    NoFileDefined,
    /// The config data is not valid
    /// The data read from the file is not valid.
    InvalidConfig,
}

/// Removes any comments from each line of the config file.
fn remove_comments(mut s: &str) -> &str {
    for i in COMMENT_CHARS.iter() {
        s = s.split(i).next().unwrap();
    }
    s
}

/// Remove any leading or trailing whitespace
fn remove_whitespace(mut value: String) -> String {
    // Remove any leading spaces
    while value.starts_with(' ') {
        value = value[1..value.len()].to_string();
    }

    // Remove any trailing spaces
    while value.ends_with(' ') {
        value = value[..value.len() - 1].to_string();
    }
    value
}

/// Config Implementation
impl Config {
    /// Create a new Config struct with all default values except for the file path.
    /// This path can be any path EX: `Sone("config.cfg")` or `NONE` if you want to load config from a string.
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::config::Config;
    ///
    /// // Create a new config with no file
    /// let mut cfg = Config::new(None);
    ///
    /// // Create a new config with a file
    /// let mut cfg2 = Config::new(Some("config.cfg"));
    /// ```
    pub fn new(file: Option<&str>) -> Self {
        let file = file.unwrap_or("");
        Config {
            file: file.to_string(),
            data: vec![],
        }
    }

    /// Reads the config file and parses it.
    /// Is similar to reading the file and passing the data to `cfg.parse();`.
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::config::Config;
    ///
    /// // Create a new config with a file
    /// let mut cfg = Config::new(Some("config.cfg"));
    ///
    /// // Read the config file
    /// cfg.read().ok().expect("Error reading the config file");
    /// ```
    pub fn read(&mut self) -> Result<Vec<[String; 2]>, ConfigError> {
        if self.file.is_empty() {
            return Err(ConfigError::NoFileDefined);
        }
        let mut contents = match fs::read_to_string(&self.file) {
            Ok(contents) => contents,
            Err(_) => return Err(ConfigError::FileReadError),
        };
        contents = contents.replace('\r', "");
        self.parse(&contents[..])
    }

    /// Parse a string as a config file.
    /// Will automatically ignore any carriage returns.
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::config::Config;
    ///
    /// // Create a new config without a file
    /// let mut cfg = Config::new(None);
    ///
    /// // Parse a string as a config file
    /// cfg.parse("hello = world\n\rrust = is great\n\rtest = \"TEST\"").ok().expect("Error parsing the config file");
    /// ```
    pub fn parse(&mut self, input_data: &str) -> Result<Vec<[String; 2]>, ConfigError> {
        let data = input_data.to_string();
        let mut done: Vec<[String; 2]> = Vec::new();

        for line in data.split('\n') {
            // Remove any space at the beginning of the line
            let mut line = &remove_whitespace(line.to_string())[..];

            // Skip empty / commented lines and sections (for now)
            match line.chars().next() {
                Some(i) if COMMENT_CHARS.contains(&&i.to_string()[..]) => continue,
                Some('[') => continue,
                None => continue,
                Some(_) => {}
            }

            // Remove any comments from the line
            line = remove_comments(line);

            // Split the line into key and value
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                return Err(ConfigError::InvalidConfig);
            }

            // Remove any spaces in the key
            let key = parts[0].replace(" ", "").to_lowercase();
            let mut value = parts[1].to_string();

            value = remove_whitespace(value);

            done.push([key, value]);
        }
        self.data = done.clone();
        Ok(done)
    }

    /// Gets a value from the config.
    /// Is not CaSe-Sensitive.
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::config::Config;
    ///
    /// // Create a new config with a file
    /// let mut cfg = Config::new(Some("config.cfg"));
    ///
    /// // Read the config file
    /// cfg.read().ok().expect("Error reading the config file");
    ///
    /// // Get a value from the config
    /// // Will Panic if the key is not found
    /// println!("Hello, {}", cfg.get("hello").unwrap());
    ///
    /// // You can set a default value if the key is not found
    /// println!("Hello, {}", cfg.get("hello").unwrap_or("Fallback".to_string()));
    /// ```
    pub fn get(&self, key: &str) -> Option<String> {
        let key = key.to_lowercase();
        for i in self.data.iter().rev() {
            if i[0] == key {
                return Some(i[1].to_string());
            }
        }
        None
    }

    /// Get a value from the config as a boolean.
    /// It checks if the value is `true` or `false`.
    /// If the value if anything else, it will return `false`.
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::config::Config;
    /// 
    /// // Create a new config from a string
    /// let mut cfg = Config::new(None);
    /// 
    /// // Parse a string as a config file
    /// cfg.parse("hello = true\nrust = false").ok().expect("Error parsing the config file");
    /// 
    /// // Get a value from the config
    /// // Will Panic if the key is not found
    /// let value = cfg.get_bool("hello").unwrap();
    /// assert_eq!(value, true);
    /// ```
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        let item = self.get(key).unwrap_or("".to_string());
        if item == "" {
            return None;
        }
        Some(item.to_lowercase() == "true")
    }

    /// Get a value from the config as an integer.
    /// Uses .parse() to parse the value to a int (i64).
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::config::Config;
    /// 
    /// // Create a new config from a string
    /// let mut cfg = Config::new(None);
    /// cfg.parse("hello = True\nrust = 15").ok().unwrap();
    /// 
    /// // Get a value from the config
    /// // Will Panic if the key is not found
    /// let hello = cfg.get_bool("hello").unwrap();
    /// assert_eq!(hello, true);
    /// 
    /// let rust = cfg.get_int("rust").unwrap();
    /// assert_eq!(rust, 15);
    /// ```
    pub fn get_int(&self, key: &str) -> Option<i64> {
        let item = self.get(key).unwrap_or("".to_string());
        if item == "" {
            return None;
        }
        Some(item.parse::<i64>().unwrap())
    }

    /// Get a value from the config as a float.
    /// Uses .parse() to parse the value to a float (f64).
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::config::Config;
    /// 
    /// // Create a new config from a string
    /// let mut cfg = Config::new(None);
    /// cfg.parse("pi = 3.1515926").ok().unwrap();
    /// 
    /// // Get a value from the config
    /// // Will Panic if the key is not found
    /// let pi = cfg.get_float("pi").unwrap();
    /// assert_eq!(pi, 3.1515926);
    /// ```
    pub fn get_float(&self, key: &str) -> Option<f64> {
        let item = self.get(key).unwrap_or("".to_string());
        if item == "" {
            return None;
        }
        Some(item.parse::<f64>().unwrap())
    }
}