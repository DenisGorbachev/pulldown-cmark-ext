use markdown::mdast::Node;
use markdown::message::Message;
use markdown::{to_mdast, ParseOptions};

pub fn mdast(input: &str) -> Result<Node, Message> {
    to_mdast(input, &ParseOptions::default())
}
