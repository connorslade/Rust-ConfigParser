#![feature(test)]

use simple_config_parser::config::Config;
extern crate test;

pub fn parse_string() {
    let mut cfg = Config::new(None);
    cfg.parse("hello = world ; Comment").ok().unwrap();
}

pub fn parse_string_get() {
    let mut cfg = Config::new(None);
    cfg.parse("hello = world ; Comment").ok().unwrap();
    cfg.get("hello");
}

pub fn parse_messy_string() {
    let mut cfg = Config::new(None);
    cfg.parse("     hello   =   world ;#;#;#; Comment")
        .ok()
        .unwrap();
}

pub fn parse_file() {
    let mut cfg = Config::new(Some("config.cfg"));
    cfg.read().ok().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    /// Basic config parsing benchmark.
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| parse_string());
    }

    #[bench]
    /// Parse string and get value.
    fn bench_parse_get(b: &mut Bencher) {
        b.iter(|| parse_string_get());
    }

    #[bench]
    /// Parse messy config string
    fn bench_parse_messy(b: &mut Bencher) {
        b.iter(|| parse_messy_string());
    }

    #[bench]
    /// Config file reading and parsing benchmark.
    fn bench_read_parse(b: &mut Bencher) {
        b.iter(|| parse_file());
    }
}
