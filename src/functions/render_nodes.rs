use markdown::mdast::Node;

pub fn render_nodes<'a>(nodes: impl IntoIterator<Item = &'a Node>) -> String {
    nodes.into_iter().map(ToString::to_string).collect()
}
