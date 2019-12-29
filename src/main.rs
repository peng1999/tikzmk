#![feature(matches_macro)]

use anyhow::{Context, Result};
use clap::{App, Arg};
use env_logger::Env;
use std::{env, fs, path, process::Command};

mod parse;
mod render;

fn main() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("warn")).init();

    let matches = App::new("TikZ preview tool")
        .arg(Arg::with_name("filepath").required(true).index(1))
        .arg(Arg::with_name("compile").short("x").long("compile"))
        .arg(
            Arg::with_name("engine")
                .short("e")
                .long("engine")
                .default_value("xelatex"),
        )
        .get_matches();

    let file_path = matches.value_of("filepath").unwrap();
    let file_text = fs::read_to_string(file_path).context("Failed to read file")?;
    let rendered = render::render(&file_text);

    if matches.is_present("compile") {
        let abs_path = fs::canonicalize(file_path).context("Path contans error")?;
        let tmp_name = abs_path
            .to_string_lossy()
            .replace(path::MAIN_SEPARATOR, "%");

        let mut tmp_dir = env::temp_dir();
        tmp_dir.push("tikzmk");
        fs::create_dir_all(tmp_dir.clone()).context("Failed to create temp directory")?;

        tmp_dir.push(tmp_name);
        fs::write(tmp_dir.clone(), rendered).context("Failed to write file")?;

        let engine = matches.value_of("engine").unwrap();
        Command::new(engine)
            .arg(tmp_dir)
            .spawn()
            .with_context(|| format!("Failed to run {}", engine))?
            .wait()?;
    } else {
        println!("{}", rendered);
    }
    Ok(())
}
