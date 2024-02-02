use std::cell::RefCell;

use comrak::{
    nodes::{Ast, AstNode, LineColumn, NodeLink, NodeValue},
    Arena,
};

// Traverse the AST, applying the provided helper function to text nodes
// The helper function can return an altered version of the text
// Certain node types are ignored and not descended into, to avoid altering text within them
fn alter_text<'a, F>(arena: &'a Arena<AstNode<'a>>, node: &'a AstNode<'a>, helper: &mut F)
where
    F: FnMut(&'a Arena<AstNode<'a>>, &str) -> Option<Vec<&'a AstNode<'a>>>,
{
    match node.data.borrow().value {
        // Ignored node types
        NodeValue::BlockQuote
        | NodeValue::List(..)
        | NodeValue::Item(..)
        | NodeValue::DescriptionItem(..)
        | NodeValue::DescriptionTerm
        | NodeValue::DescriptionDetails
        | NodeValue::DescriptionList
        | NodeValue::CodeBlock(..)
        | NodeValue::HtmlBlock(..)
        | NodeValue::Table(..)
        | NodeValue::ThematicBreak
        | NodeValue::Strong
        | NodeValue::Emph
        | NodeValue::Link(..)
        | NodeValue::Image(..) => (),
        // For paragraph nodes, apply the helper function to the text nodes within it
        NodeValue::Paragraph => {
            for c in node.children() {
                if let NodeValue::Text(literal) = &c.data.borrow().value {
                    if let Some(v) = helper(arena, literal) {
                        // if helper returns something, replace the current node with
                        // what was returned.
                        for item in v.into_iter() {
                            c.insert_before(item);
                        }
                        c.detach()
                    }
                }
            }
        }
        // For all other node types, recurse into their children
        _ => {
            for c in node.children() {
                alter_text(arena, c, helper);
            }
        }
    }
}

pub fn add_links<'a, F>(arena: &'a Arena<AstNode<'a>>, node: &'a AstNode<'a>, mut helper: F)
where
    F: FnMut(&str) -> Vec<(usize, usize, String)>,
{
    alter_text(arena, node, &mut |arena, text: &str| {
        let mut new_nodes: Vec<&AstNode> = vec![];

        let mut links = helper(text);
        links.sort();

        if links.is_empty() {
            return None;
        }

        let segment = |start, end| {
            let segment = &text[start..end];
            let segment_node = AstNode::new(RefCell::new(Ast::new(
                NodeValue::Text(segment.to_string()),
                LineColumn { line: 1, column: 1 },
            )));
            arena.alloc(segment_node)
        };

        let mut cur = 0;
        for (start, length, url) in links {
            if cur < start {
                new_nodes.push(segment(cur, start));
            }

            let link = NodeLink {
                url,
                title: "".to_string(),
            };
            let link_node = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                NodeValue::Link(link),
                LineColumn { line: 1, column: 1 },
            ))));
            // Add a Google text node as a child of the link node
            let text_node = segment(start, start + length);
            link_node.prepend(text_node);
            new_nodes.push(link_node);
            cur = start + length;
        }

        if cur < text.len() {
            new_nodes.push(segment(cur, text.len()));
        }

        if new_nodes.is_empty() {
            None
        } else {
            Some(new_nodes)
        }
    });
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use comrak::{
        nodes::{Ast, AstNode, LineColumn, NodeLink, NodeValue},
        parse_document, Arena, ComrakOptions,
    };

    use super::*;

    // Test that the helper function correctly capitalizes all text in paragraphs
    #[test]
    fn test_capitalize_paragraphs() {
        let markdown = "
# Heading

This is a paragraph. **Testing Testing**

Another paragraph.";

        let arena = Arena::new();
        let root = parse_document(&arena, markdown, &ComrakOptions::default());

        alter_text(&arena, root, &mut |arena, text: &str| {
            let capitalized_text = text.to_uppercase();
            let new_node = AstNode::new(RefCell::new(Ast::new(
                NodeValue::Text(capitalized_text),
                LineColumn { line: 1, column: 1 },
            )));
            Some(vec![arena.alloc(new_node)])
        });

        let mut buf = vec![];
        comrak::format_commonmark(root, &ComrakOptions::default(), &mut buf).unwrap();
        let result = String::from_utf8(buf).unwrap();

        assert!(result.contains("THIS IS A PARAGRAPH."));
        assert!(result.contains("ANOTHER PARAGRAPH."));
        assert!(result.contains("Testing Testing"));
    }

    // Test that the helper function correctly replaces occurrences of 'Google' with a link
    #[test]
    fn test_link_google() {
        let markdown = "
# Heading

This is a paragraph with Google.

Another paragraph.";

        let arena = Arena::new();
        let root = parse_document(&arena, markdown, &ComrakOptions::default());

        alter_text(&arena, root, &mut |arena, text: &str| {
            let mut new_nodes: Vec<&AstNode> = vec![];

            for (i, segment) in text.split("Google").enumerate() {
                // If this is not the first segment, prepend a Google link
                if i != 0 {
                    let link = NodeLink {
                        url: "https://www.google.com".to_string(),
                        title: "".to_string(),
                    };
                    let link_node = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                        NodeValue::Link(link),
                        LineColumn { line: 1, column: 1 },
                    ))));
                    // Add a Google text node as a child of the link node
                    let google_text_node = arena.alloc(AstNode::new(RefCell::new(Ast::new(
                        NodeValue::Text("Google".to_string()),
                        LineColumn { line: 1, column: 1 },
                    ))));
                    link_node.prepend(google_text_node);
                    new_nodes.push(link_node);
                }

                // Add a new text node for this segment
                let segment_node = AstNode::new(RefCell::new(Ast::new(
                    NodeValue::Text(segment.to_string()),
                    LineColumn { line: 1, column: 1 },
                )));
                new_nodes.push(arena.alloc(segment_node));
            }

            if new_nodes.is_empty() {
                None
            } else {
                Some(new_nodes)
            }
        });

        let mut buf = vec![];
        comrak::format_commonmark(root, &ComrakOptions::default(), &mut buf).unwrap();
        let result = String::from_utf8(buf).unwrap();

        assert!(result.contains("This is a paragraph with [Google](https://www.google.com)."));
        assert!(result.contains("Another paragraph."));
    }
}
