#![feature(test)]

extern crate test;
use test::Bencher;

use simple_config_parser::Config;

pub fn parse_string() -> Option<()> {
    let cfg = Config::new().text("hello = world ; Comment").ok()?;

    Some(())
}

pub fn parse_string_get() -> Option<()> {
    let cfg = Config::new().text("hello = world ; Comment").ok()?;
    cfg.get_str("hello").unwrap();
    Some(())
}

pub fn parse_messy_string() -> Option<()> {
    let cfg = Config::new()
        .text("     hello   =   world ;#;#;#; Comment")
        .ok()?;

    Some(())
}

pub fn parse_file() -> Option<()> {
    let cfg = Config::new().file("config.cfg").ok()?;

    Some(())
}

pub fn parse_string_get_bool() -> Option<()> {
    let cfg = Config::new().text("hello = true ; Comment").ok()?;
    cfg.get::<bool>("hello").ok()?;

    Some(())
}

pub fn parse_string_get_int() -> Option<()> {
    let cfg = Config::new().text("hello = 1 ; Comment").ok()?;
    cfg.get::<i32>("hello").ok()?;

    Some(())
}

pub fn parse_string_get_float() -> Option<()> {
    let mut cfg = Config::new().text("hello = 1.0 ; Comment").ok()?;
    cfg.get::<f32>("hello").ok()?;

    Some(())
}

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

#[bench]
/// Parse string and get bool value.
fn bench_parse_get_bool(b: &mut Bencher) {
    b.iter(|| parse_string_get_bool());
}

#[bench]
/// Parse string and get bool value.
fn bench_parse_get_int(b: &mut Bencher) {
    b.iter(|| parse_string_get_int());
}

#[bench]
/// Parse string and get bool value.
fn bench_parse_get_float(b: &mut Bencher) {
    b.iter(|| parse_string_get_float());
}
