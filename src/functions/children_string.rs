use markdown::mdast::Node;

pub fn inner_text(node: &Node) -> Option<String> {
    node.children()
        .map(|children| children.iter().map(ToString::to_string).collect())
}
