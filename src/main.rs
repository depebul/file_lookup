mod cli;
mod display;
mod search;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let results = search::execute_search(&cli)?;
    display::show_results(&results, &cli);

    Ok(())
}
