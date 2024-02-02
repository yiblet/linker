mod bigram;
mod document;
mod front_matter;
mod index;
mod keyword;
mod markdown;
mod ngram;
mod write;

use clap::Parser;
use std::path::PathBuf;

/// A program to auto-link a glob of markdowns. 
/// All markdowns must have the following front matter: 
/// --- 
/// keywords: 
///     - <key1>
///     - <key2> 
/// slug: <slug>
/// ---
///
/// The program will read in all keywords and slugs from all markdowns 
/// and then identify uses of those keywords throughout the 
/// glob of markdowns and create links accordingly.
#[derive(Parser)]
struct Args {
    /// the glob of markdowns affected
    glob: String, 
    /// the path to the folder preserving folder structure
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::init();
    let mut keywords = keyword::Keywords::new();

    log::info!("indexing...");
    index::index(&mut keywords, &args.glob)?;
    log::info!("updating...");
    write::write_glob(&keywords, &args.glob, &args.output)?;

    Ok(())
}
