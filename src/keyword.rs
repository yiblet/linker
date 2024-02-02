use std::collections::{HashMap, HashSet};

type Slug = String;
type Keyword = String;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Keywords {
    pub(crate) map: HashMap<Keyword, HashSet<Slug>>,
}

impl Keywords {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    pub fn insert<K: AsRef<str>>(&mut self, slug: &str, keywords: &[K]) {
        let slug = slug.trim();
        for keyword in keywords {
            let keyword = keyword.as_ref().to_lowercase();
            let slugs = self.map.entry(keyword).or_insert(Default::default());
            slugs.insert(slug.to_string());
        }
    }

    pub fn get(&self, keyword: &str) -> Option<impl Iterator<Item = &str>> {
        let set = self.map.get(keyword)?;
        Some(set.iter().map(String::as_ref))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords_new() {
        // This function tests the "new" method which should create an empty "Keywords" instance
        let keywords = Keywords::new();
        assert_eq!(keywords.map, HashMap::new());
    }

    #[test]
    fn test_keywords_insert() {
        // This function tests the "insert" method which inserts a keyword association to a slug
        let mut keywords = Keywords::new();
        keywords.insert("example-slug", &["example_keyword_1", "example_keyword_2"]);

        let keyword_1_slugs = keywords.map.get("example_keyword_1").unwrap();
        let keyword_2_slugs = keywords.map.get("example_keyword_2").unwrap();

        assert_eq!(keyword_1_slugs.len(), 1);
        assert_eq!(keyword_2_slugs.len(), 1);
        assert!(keyword_1_slugs.contains("example-slug"));
        assert!(keyword_2_slugs.contains("example-slug"));
    }

    #[test]
    fn test_keywords_get() {
        // This function tests the "get" method which retrieves a list of slugs associated to a keyword
        let mut keywords = Keywords::new();
        keywords.insert("example-slug", &["example_keyword"]);

        let slugs = keywords.get("example_keyword").unwrap().collect::<Vec<_>>();
        assert_eq!(slugs.len(), 1);
        assert_eq!(slugs[0], "example-slug");
    }

    #[test]
    fn test_keywords_get_no_keyword() {
        // This function tests the "get" method when a keyword does not exist in the map
        let keywords = Keywords::new();
        assert!(keywords.get("non-existent-keyword").is_none());
    }
}
