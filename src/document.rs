#[derive(Debug, serde::Deserialize, Clone)]
pub struct FrontMatter<'a> {
    pub slug: &'a str,
    pub keywords: Vec<&'a str>,
}

impl<'a> FrontMatter<'a> {
    fn parse(content: &'a str) -> anyhow::Result<Self> {
        let parsed_fm: FrontMatter = serde_yaml::from_str(content)?;
        Ok(parsed_fm)
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Document<'a> {
    pub front_matter: FrontMatter<'a>,
    pub front_matter_all: &'a str,
    pub document: &'a str,
}

impl<'a> Document<'a> {
    pub fn parse(content: &'a str) -> anyhow::Result<Self> {
        let (fm, md) = match crate::front_matter::split_front_matter(content) {
            Some(fm_md) => fm_md,
            None => return Err(anyhow::anyhow!("Failed to parse front matter")),
        };
        let front_matter = FrontMatter::parse(fm)?;

        Ok(Self {
            front_matter,
            front_matter_all: fm,
            document: md,
        })
    }
}
