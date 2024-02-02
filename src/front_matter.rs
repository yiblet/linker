pub fn split_front_matter(input: &str) -> Option<(&str, &str)> {
    let ("---", next) = input.trim_start().split_at("---".len()) else {
        return None;
    };

    let Some((front_matter, markdown)) = next.split_once("\n---\n") else {
        return None;
    };

    Some((front_matter, markdown))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_front_matter() {
        let input = "
---
title: A sample title
description: A sample description
---duplicate
---

This is the main content.";

        let (front_matter, markdown) = split_front_matter(input).unwrap();

        assert_eq!(
            front_matter.trim(),
            "title: A sample title\ndescription: A sample description\n---duplicate"
        );
        assert_eq!(markdown.trim(), "This is the main content.");
    }

    #[test]
    fn test_split_front_matter_no_front_matter() {
        let input = "This is a content without front matter.";

        let result = split_front_matter(input);

        assert!(result.is_none());
    }

    #[test]
    fn test_split_front_matter_empty_front_matter() {
        let input = "---
---

This is the main content with empty front matter.";

        let (front_matter, markdown) = split_front_matter(input).unwrap();

        assert_eq!(front_matter, "");
        assert_eq!(
            markdown.trim(),
            "This is the main content with empty front matter."
        );
    }
}
