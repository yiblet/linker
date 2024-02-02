use std::io::Read;

use crate::{document, keyword};

pub fn index(keywords: &mut keyword::Keywords, glob_str: &str) -> anyhow::Result<()> {
    // Glob for markdown files
    for entry in glob::glob(glob_str)? {
        let path = entry?;
        // Read the file content
        let mut file = std::fs::File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        if let Err(err) = index_content(keywords, &content) {
            log::warn!(
                "file {} cannot not be indexed: {}",
                &path.to_string_lossy(),
                err
            )
        }
    }
    Ok(())
}

fn index_content(keywords: &mut keyword::Keywords, content: &str) -> Result<(), document::Error> {
    let doc = document::Document::parse(content)?;
    keywords.insert(doc.front_matter.slug, &doc.front_matter.keywords);
    Ok(())
}
