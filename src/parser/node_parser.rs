use crate::parser::tag_parser::{Property, TagNode};

#[derive(Debug)]
pub enum Node {
    Element {
        tag: String,
        children: Vec<Node>,
        properties: Vec<Property>,
    },
    VoidElement {
        tag: String,
        properties: Vec<Property>,
    },
    Text(String),
}

pub struct NodeParser {
    tags: Vec<TagNode>,
    pos: usize,
}

impl NodeParser {
    pub fn new(tags: Vec<TagNode>) -> Self {
        Self { tags, pos: 0 }
    }

    fn peek(&self) -> Option<&TagNode> {
        self.tags.get(self.pos)
    }

    fn advance(&mut self) -> Option<TagNode> {
        let node = self.peek()?.clone();
        self.pos += 1;
        Some(node)
    }

    fn expect<F>(&mut self, cond: F) -> bool
    where
        F: Fn(&TagNode) -> bool,
    {
        if let Some(tag) = self.peek() {
            if cond(tag) {
                self.pos += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = vec![];

        while self.tags.len() > self.pos {
            if let Some(node) = self.parse_node() {
                nodes.push(node);
            } else {
                break;
            }
        }

        nodes
    }

    fn parse_node(&mut self) -> Option<Node> {
        let tag = self.advance()?;

        match tag {
            TagNode::OpenTag { name, properties } => {
                let mut children = vec![];
                while !self.expect(|tag| tag == &TagNode::CloseTag(name.clone())) {
                    children.push(self.parse_node()?);
                }
                Some(Node::Element {
                    tag: name,
                    children,
                    properties,
                })
            }
            TagNode::VoidTag { name, properties } => Some(Node::VoidElement {
                tag: name,
                properties,
            }),
            TagNode::Text(text) => Some(Node::Text(text)),
            _ => None,
        }
    }
}
