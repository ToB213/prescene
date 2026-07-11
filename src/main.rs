mod error;
mod model;
mod renderer;

use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::error::AppError;
use crate::model::Presentation;

#[derive(Debug, Parser)]
#[command(name = "prescene", about = "A simple presentation generator")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Build {
        input: PathBuf,

        #[arg(short, long, default_value = "output/index.html")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(cli) {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Command::Build { input, output } => build(input, output),
    }
}

fn build(input: PathBuf, output: PathBuf) -> Result<(), AppError> {
    let source = fs::read_to_string(&input).map_err(|source| AppError::ReadFile {
        path: input.clone(),
        source,
    })?;

    let presentatoin: Presentation = serde_yaml::from_str(&source)?;
    let html = renderer::render_html(&presentatoin);

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|source| AppError::WriteFile {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    fs::write(&output, html).map_err(|source| AppError::WriteFile {
        path: output.clone(),
        source,
    })?;

    println!("generated {}", output.display());

    Ok(())
}
