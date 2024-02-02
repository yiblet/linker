use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::fs::{self, create_dir_all};
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::{document, keyword, markdown, ngram};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Candidate {
    start: usize,
    length: usize,
    keyword: String,
    url: String,
}

impl Candidate {
    fn end(&self) -> usize {
        self.start + self.length
    }

    fn tuple(self) -> (usize, usize, String) {
        (self.start, self.length, self.url)
    }
}

fn generate_candidates<R: Rng>(
    keywords: &keyword::Keywords,
    paragraph: &str,
    doc: &document::Document,
    rng: &mut R,
) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    for n in (2..=4).rev() {
        for grams in ngram::ngram(
            ngram::positioned(paragraph, paragraph.split_whitespace()),
            n,
        ) {
            let word = itertools::join(grams.iter().map(|s| s.1.to_lowercase()), " ");

            if let Some(slugs) = keywords.get(&word) {
                let slugs: Vec<_> = slugs
                    .filter(|slug| *slug != doc.front_matter.slug)
                    .collect();

                if slugs.is_empty() {
                    continue;
                }

                let slug = slugs[rng.next_u64() as usize % slugs.len()];
                if let Some((start, len)) =
                    grams.first().and_then(|start| -> Option<(usize, usize)> {
                        let end = grams.last()?;
                        Some((start.0, end.0 + end.1.len() - start.0))
                    })
                {
                    candidates.push(Candidate {
                        start,
                        length: len,
                        keyword: word,
                        url: slug.to_owned(),
                    });
                }
            }
        }
    }
    candidates
}

// TODO: returning the String is an additional allocation that is not necessary
fn update_content(keywords: &keyword::Keywords, content: &str) -> anyhow::Result<String> {
    // helper functions
    let is_already_seen =
        |candidate: &Candidate, seen_keywords: &HashSet<String>, seen_urls: &HashSet<String>| {
            seen_urls.contains(&candidate.url) || seen_keywords.contains(&candidate.keyword)
        };

    // set up
    let doc = document::Document::parse(content)?;
    let arena = comrak::Arena::new();
    let ast = comrak::parse_document(&arena, doc.document, &Default::default());
    let mut rng = rand::thread_rng(); // used to sample a candidate if there are multiple available

    let mut added_url: HashSet<String> = HashSet::new();
    let mut added_keyword: HashSet<String> = HashSet::new();
    markdown::add_links(&arena, ast, |paragraph| {
        let mut candidates = generate_candidates(keywords, paragraph, &doc, &mut rng);

        if candidates.is_empty() {
            return vec![];
        }

        candidates.sort_by(|a, b| match a.start.cmp(&b.start) {
            std::cmp::Ordering::Equal => b.length.cmp(&a.length),
            otherwise => otherwise,
        });

        let mut res: Vec<Candidate> = Vec::new();

        let mut seen_keywords: HashSet<String> = HashSet::new();
        let mut seen_urls: HashSet<String> = HashSet::new();

        for candidate in candidates {
            if is_already_seen(&candidate, &seen_keywords, &seen_urls)
                || is_already_seen(&candidate, &added_keyword, &added_url)
            {
                continue;
            }

            match res.last() {
                Some(c) if c.end() > candidate.start => continue, // skip overlaps
                _ => {
                    seen_urls.insert(candidate.url.clone());
                    seen_keywords.insert(candidate.keyword.clone());
                    res.push(candidate);
                }
            }
        }

        res.shuffle(&mut rng);
        if res.len() > 4 {
            res.drain(4..);
        }

        added_url.extend(res.iter().map(|c| c.url.clone()));
        added_keyword.extend(res.iter().map(|c| c.keyword.clone()));
        res.into_iter().map(Candidate::tuple).collect()
    });

    let mut out = Vec::with_capacity(content.len());
    write!(&mut out, "---\n{}\n---\n", doc.front_matter_all.trim(),)?;
    comrak::format_commonmark(ast, &Default::default(), &mut out)?;

    Ok(String::from_utf8(out)?)
}

pub fn write_glob(
    keywords: &keyword::Keywords,
    glob_str: &str,
    output: &Path,
) -> anyhow::Result<()> {
    // Glob for markdown files
    for entry in glob::glob(glob_str)? {
        if glob_str.contains("..") {
            return Err(anyhow::anyhow!("cannot glob on \"..\" directories"));
        }
        let path = entry?;

        let output_path = output.join(&path);

        // Read the file content
        let mut file = fs::File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        log::info!("updating {}", path.to_string_lossy());

        let updated_file = match update_content(keywords, &content) {
            Ok(file) => file,
            Err(err) => match err.downcast_ref::<document::Error>() {
                Some(doc_err) => {
                    log::warn!(
                        "skipped updating {} since it's missing keywords or slugs: {}",
                        path.to_string_lossy(),
                        doc_err
                    );
                    content
                }
                None => Err(err)?,
            },
        };

        if let Some(parent) = output_path.parent() {
            create_dir_all(parent)?;
        }

        let mut out = fs::File::create(&output_path)?;
        out.write_all(updated_file.as_bytes())?;
    }
    Ok(())
}
