#![feature(matches_macro)]

use anyhow::{anyhow, Context, Result};
use clap::{App, Arg};
use env_logger::Env;
use log::info;
use std::{env, fs, path, process::Command};

mod parse;
mod render;

fn main() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let matches = App::new("TikZ preview tool")
        .arg(Arg::with_name("filepath").required(true).index(1))
        .arg(Arg::with_name("compile").short("x").long("compile"))
        .arg(
            Arg::with_name("engine")
                .short("e")
                .long("engine")
                .default_value("xelatex"),
        )
        .arg(Arg::with_name("open").long("open").requires("compile"))
        .get_matches();

    let file_path = matches.value_of("filepath").unwrap();
    let file_text = fs::read_to_string(file_path).context("Failed to read file")?;
    let rendered = render::render(&file_text);

    if matches.is_present("compile") {
        let engine = matches.value_of("engine").unwrap();
        let shoud_open = matches.is_present("open");
        write_tmp_and_compile(file_path, &rendered, engine, shoud_open)
            .context("Failed to compile")?;
    } else {
        println!("{}", rendered);
    }
    Ok(())
}

fn write_tmp_and_compile(
    file_path: &str,
    rendered: &str,
    engine: &str,
    shoud_open: bool,
) -> Result<()> {
    let abs_path = fs::canonicalize(file_path).context("Path contans error")?;
    let tmp_name = abs_path
        .to_string_lossy()
        .replace(path::MAIN_SEPARATOR, "_");

    let mut tmp_dir = env::temp_dir();
    tmp_dir.push("tikzmk");
    fs::create_dir_all(&tmp_dir).context("Failed to create temp directory")?;

    tmp_dir.push(&tmp_name);
    fs::write(&tmp_dir, rendered).context("Failed to write file")?;

    info!("Running {}", engine);
    let output = Command::new(engine)
        .arg("-interaction")
        .arg("batchmode")
        .arg(&tmp_name)
        .current_dir(&tmp_dir.parent().unwrap())
        .output()
        .with_context(|| format!("Failed to run {}", engine))?;

    if !output.status.success() {
        return Err(anyhow!(
            "{} returns error: {}",
            engine,
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    if shoud_open {
        info!("Opening pdf");
        tmp_dir.set_extension("pdf");
        open::that(&tmp_dir).context("Failed to open")?;
    }

    Ok(())
}
