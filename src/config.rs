//! This module contains the things needed to load and parse ini like configuration files
use std::fs;
use std::path::Path;

/// Define valid comment chars.
const COMMENT_CHARS: [&str; 2] = ["#", ";"];

/// Config Struct
pub struct Config {
    /// Raw Data of the Config
    pub data: Vec<[String; 2]>,
}

/// Some errors that can be thrown by this module
#[derive(Debug)]
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
    /// Error Parseing config value into specified type
    ParseError,
    /// No item for the key provided exists
    NoItem,
}

/// Removes any comments from each line of the config file.
fn remove_comments(str: String) -> String {
    let mut s = str.as_str();
    for i in COMMENT_CHARS.iter() {
        s = s.split(i).next().unwrap();
    }
    s.to_string()
}

/// Config Implementation
impl Config {
    /// Create a new Config struct
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::Config;
    ///
    /// let mut cfg = Config::new();
    /// ```
    pub fn new() -> Self {
        Config { data: Vec::new() }
    }

    /// Reads and parses config from a file
    ///
    /// If called more than one time it will append the current values.
    /// So the recently appended valued will take priority
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::Config;
    ///
    /// // Create a new config with a file
    /// let mut cfg = Config::new().file("config.cfg").unwrap();
    ///
    /// // Read a value
    /// assert_eq!(cfg.get_str("hello").unwrap(), "World");
    /// ```
    pub fn file<T>(self, file: T) -> Result<Self, ConfigError>
    where
        T: AsRef<Path>,
    {
        let contents = match fs::read_to_string(file) {
            Ok(contents) => contents,
            Err(_) => return Err(ConfigError::FileReadError),
        };

        let mut data = self.data;
        data.append(&mut Config::parse(contents)?);

        Ok(Self { data })
    }

    /// Parses config from text or anything that impls fmt::Display
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::Config;
    ///
    /// // Create a new config with text
    /// let mut cfg = Config::new().text("hello = World").unwrap();
    ///
    /// // Read a value
    /// assert_eq!(cfg.get_str("hello").unwrap(), "World");
    /// ```
    pub fn text<T>(self, text: T) -> Result<Self, ConfigError>
    where
        T: std::fmt::Display,
    {
        let data = Config::parse(text.to_string())?;

        Ok(Self { data })
    }

    /// Get a value from config as ayn type (That Impls str::FromStr)
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::Config;
    ///
    /// // Create a new config with text
    /// // I knew a lot more of pi at one point :P
    /// let mut cfg = Config::new().text("pi = 3.14159265358979").unwrap();
    ///
    /// // Read a value
    /// assert_eq!(cfg.get::<f32>("pi").unwrap(), 3.14159265358979);
    /// ```
    pub fn get<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: core::str::FromStr,
    {
        let key = key.to_string().to_lowercase();
        for i in self.data.iter().rev() {
            if i[0] != key {
                continue;
            }
            match i[1].parse() {
                Ok(i) => return Ok(i),
                Err(_) => return Err(ConfigError::ParseError),
            }
        }

        Err(ConfigError::NoItem)
    }

    /// Get a value from config as a String
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use simple_config_parser::Config;
    ///
    /// // Create a new config with text
    /// let mut cfg = Config::new().text("pi = 3.14159265358979").unwrap();
    ///
    /// // Read a value
    /// assert_eq!(cfg.get_str("pi").unwrap(), "3.14159265358979");
    /// ```
    pub fn get_str(&self, key: &str) -> Result<String, ConfigError> {
        let key = key.to_string().to_lowercase();
        for i in self.data.iter().rev() {
            if i[0] != key {
                continue;
            }
            return Ok(i[1].to_string());
        }

        Err(ConfigError::NoItem)
    }

    /// Parse a string into the config
    fn parse(input_data: String) -> Result<Vec<[String; 2]>, ConfigError> {
        let mut done: Vec<[String; 2]> = Vec::new();

        for line in input_data.lines() {
            // Remove any space at the beginning of the line
            let mut line = line.trim().to_string();

            // Skip empty / commented lines and sections (for now)
            match line.chars().next() {
                Some(i) if COMMENT_CHARS.contains(&&i.to_string()[..]) => continue,
                Some('[') => continue,
                Some(_) => {}
                None => continue,
            }

            // Remove any comments from the line
            line = remove_comments(line.to_string());

            // Split the line into key and value
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                return Err(ConfigError::InvalidConfig);
            }

            // Remove any spaces in the key
            let key = parts[0].replace(" ", "").to_lowercase();
            let value = parts[1].trim().to_string();

            done.push([key, value]);
        }

        Ok(done)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}
