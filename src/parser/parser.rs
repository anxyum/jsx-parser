use crate::lexer::Token;
use crate::parser::tag_parser::{Property, TagNode};

#[derive(Debug)]
enum Node {
    Element {
        tag: String,
        children: Vec<Node>,
        properties: Vec<Property>,
    },
    Text(String),
}

pub struct Parser {
    tags: Vec<TagNode>,
    pos: usize,
}

impl Parser {
    pub fn new(tags: Vec<TagNode>) -> Self {
        Self { tags, pos: 0 }
    }

    fn cleanup_stream(&mut self) {
        if self.token_stream.len() == 0 {
            return;
        }
        let mut inside_text = false;
        let mut cleaned_token_stream = vec![];

        for i in 0..(self.token_stream.len() - 1) {
            let token = self.token_stream.get(i).unwrap();
            let next = self.token_stream.get(i + 1).unwrap();
            if !inside_text {
                match next {
                    Token::OpenAngle => inside_text = false,
                    _ => (),
                }
                match token {
                    Token::Text(_) => inside_text = true,
                    _ => (),
                }
            } else {
                match token {
                    Token::WhiteSpace => continue,
                    _ => (),
                }
            }
            cleaned_token_stream.push(token.clone());
        }

        self.token_stream = cleaned_token_stream;
    }

    fn peek(&self) -> Option<&Token> {
        self.token_stream.get(self.pos)
    }

    fn next_token(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.peek()
    }

    pub fn parse(mut self) -> Option<Node> {
        self.cleanup_stream();

        let mut children = Vec::new();

        while self.token_stream.len() > self.pos {
            if let Some(node) = self.parse_node() {
                children.push(node);
            }
        }

        Some(Node::Element {
            children,
            properties: vec![],
            tag: String::from("root"),
        })
    }

    fn parse_node(&mut self) -> Option<Node> {
        let token = self.next_token()?.clone();

        match token {
            Token::OpenAngle => {
                let node = self.parse_tag()?;
                Some(node)
            }
            Token::Text(text) => return self.parse_text(&text),
            _ => None,
        }
    }

    fn parse_tag(&mut self) -> Option<(String, Vec<Property>)> {
        let token = self.next_token()?.clone();

        let tag;
        let mut properties = vec![];

        match token {
            Token::Identifier(text) => {
                tag = text;
                while let Some(token) = self.peek() {
                    match token {
                        Token::Identifier(text) => {
                            self.next_token();
                            match self.peek() {
                                _ => (),
                            };
                        }
                        _ => break,
                    }
                }
            }
            _ => return None,
        }

        Some((tag, properties))
    }

    fn parse_text(&mut self, start: &String) -> Option<Node> {
        None
    }
}
