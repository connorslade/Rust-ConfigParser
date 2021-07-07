use std::fs;

/// Config Struct
pub struct Config {
    pub file: String,
    data: Vec<[String; 2]>,
}

/// Some errors that can be thrown by this module
pub enum ConfigError {
    /// Error reading the file from disk
    /// Could have been caused by the file not existing or being inaccessible.
    FileReadError,
    /// File path has not been defined
    /// You need to define the path to the config file before using this function.
    /// Or just use `cfg.parse(<STRING>);` instead.
    NoFileDefined,
    /// The config data is not valid
    /// The data read from the file is not valid.
    InvalidConfig,
}

/// Config Implementation
impl Config {
    /// Default Config
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

    /// Parse a string as a config file
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
            // Skip empty / commented lines
            match line.chars().next() {
                Some('#') => continue,
                Some(';') => continue,
                None => continue,
                Some(_) => {}
            }
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                return Err(ConfigError::InvalidConfig);
            }
            let key = parts[0].replace(" ", "").to_lowercase();
            let mut value = parts[1].to_string();

            while value.starts_with(' ') {
                value = value[1..value.len()].to_string();
            }

            done.push([key, value]);
        }
        self.data = done.clone();
        Ok(done)
    }

    /// Gets a value from the config.
    /// Is CaSe-Sensitive.
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
    /// println!("Hello, {}", cfg.get("hello"));
    /// ```
    pub fn get(&self, key: &str) -> String {
        let key = key.to_lowercase();
        for i in self.data.iter() {
            if i[0] == key {
                return i[1].to_string();
            }
        }
        "".to_string()
    }
}