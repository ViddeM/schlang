use clap::Parser;
use eyre::Context;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The file to run.
    // #[arg(short, long, value_name = "FILE")]
    source_file: PathBuf,
}
fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let program_string =
        fs::read_to_string(cli.source_file).wrap_err("Failed to read source file")?;

    schlang_lib::run(&program_string);

    Ok(())
}
