#![feature(matches_macro)]

use anyhow::{Context, Result};
use clap::{App, Arg};
use env_logger::Env;
use std::fs;

mod parse;
mod render;

fn main() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("warn")).init();

    let matches = App::new("TikZ preview tool")
        .arg(Arg::with_name("filepath").required(true).index(1))
        .get_matches();

    let file_path = matches.value_of("filepath").unwrap();
    let file_text = fs::read_to_string(file_path).context("Failed to read file")?;
    let rendered = render::render(&file_text);
    println!("{}", rendered);
    Ok(())
}
