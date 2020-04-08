#![cfg_attr(test, feature(matches_macro))]

#[macro_use]
extern crate pest_derive;

use anyhow::{anyhow, Context, Result};
use structopt::{StructOpt, };
use env_logger::Env;
use log::info;
use std::{env, fs, path, process::Command};

mod parse;
mod render;
mod parse_pest;

#[derive(Debug, StructOpt)]
#[structopt(about = "TikZ preview tool")]
struct Opt {
    /// input file
    #[structopt(index = 1, required = true)]
    file_path: String,

    /// compile file
    #[structopt(short = "x", long)]
    compile: bool,

    /// TeX engine
    #[structopt(short = "e", long, default_value = "xelatex")]
    engine: String,

    /// open a viewer
    #[structopt(long, requires = "compile")]
    open: bool,
}

fn main() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let opt = Opt::from_args();

    let file_text = fs::read_to_string(&opt.file_path).context("Failed to read file")?;
    let rendered = render::render(&file_text);

    if opt.compile {
        write_tmp_and_compile(&opt.file_path, &rendered, &opt.engine, opt.open)
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
    let output = Command::new("texfot")
        .arg(engine)
        .arg("-interaction=nonstopmode")
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
