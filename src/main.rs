mod document;
mod front_matter;
mod index;
mod keyword;
mod markdown;
mod ngram;
mod update;
mod bigram;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    glob: String,
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::init();
    let mut keywords = keyword::Keywords::new();

    log::info!("indexing...");
    index::index(&mut keywords, &args.glob)?;
    log::info!("updating...");
    update::update(&keywords, &args.glob, &args.output)?;

    Ok(())
}
