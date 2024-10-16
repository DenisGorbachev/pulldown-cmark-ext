use crate::inner_text;
use markdown::mdast::{Heading, Node};

/// Returns the heading that matches `name` and every node after that heading until it encounters a heading of the same or lower level
pub fn section<'a, 'b>(name: &'a str, node: &'b Node) -> Option<(&'b Heading, Vec<&'b Node>)> {
    let mut heading_opt: Option<(&'b Heading, Vec<&'b Node>)> = None;

    if let Some(children) = node.children() {
        for child in children {
            match child {
                node @ Node::Heading(heading_new) => {
                    if let Some((heading_current, nodes_current)) = heading_opt.as_mut() {
                        if heading_new.depth <= heading_current.depth {
                            break;
                        } else {
                            nodes_current.push(node)
                        }
                    } else if let Some(inner_text) = inner_text(node) {
                        if inner_text == name {
                            heading_opt = Some((heading_new, vec![]))
                        }
                    }
                }
                other => {
                    if let Some((_heading_current, nodes_current)) = heading_opt.as_mut() {
                        nodes_current.push(other)
                    }
                }
            }
        }
    }

    heading_opt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mdast, render_nodes};
    use indoc::indoc;

    #[test]
    #[ignore]
    fn must_get_section() {
        let input = include_str!("../../fixtures/regular.md");
        let node = mdast(input).unwrap();
        let (heading, children) = section("Heading 1.1", &node).unwrap();
        let section_text = render_section(heading.clone(), children);
        assert_eq!(
            section_text,
            indoc! {"
            ## Heading 1.1

            Text 1.2

            ```rust
            fn main() {}
            ```
        "}
        );
    }

    /// Requires taking ownership of the `heading` because it has to be wrapped in `Node::Heading` because `Heading` by itself doesn't implement `ToString`
    fn render_section<'b>(heading: Heading, nodes: impl IntoIterator<Item = &'b Node>) -> String {
        let heading = Node::Heading(heading).to_string();
        let nodes = render_nodes(nodes);
        format!("{heading}\n\n{nodes}")
    }
}
