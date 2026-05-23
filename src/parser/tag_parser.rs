use crate::lexer::Token;
use crate::parser::TokenStream;

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TagNode {
    OpenTag {
        name: String,
        properties: Vec<Property>,
    },
    CloseTag(String),
    VoidTag {
        name: String,
        properties: Vec<Property>,
    },
    Text(String),
}

pub struct TagParser {
    tokens: TokenStream,
}

impl TagParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: TokenStream::new(tokens),
        }
    }

    pub fn parse(&mut self) -> Vec<TagNode> {
        let mut tag_nodes = Vec::new();

        while !self.tokens.finished() {
            if let Some(node) = self.parse_tag() {
                tag_nodes.push(node);
            } else {
                break;
            }
        }

        tag_nodes
    }

    fn parse_tag(&mut self) -> Option<TagNode> {
        self.tokens.skip_whitespace();

        let token = self.tokens.advance()?;

        match token {
            Token::OpenAngle => {
                let mut tokens = self
                    .tokens
                    .read_until(|tok| tok == Token::CloseAngle)
                    .remove_whitespaces();
                match tokens.advance()? {
                    Token::Slash => match tokens.advance()? {
                        Token::Identifier(ident) => {
                            if tokens.expect(Token::CloseAngle) {
                                Some(TagNode::CloseTag(ident))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Token::Identifier(ident) => {
                        let mut properties = vec![];
                        while !tokens.finished() {
                            let token = tokens.advance()?;
                            match token {
                                Token::Identifier(prop) => {
                                    let as_value = tokens.expect(Token::Equals);
                                    properties.push(if as_value {
                                        let token = tokens.advance()?;
                                        match token {
                                            Token::String(value) => Property {
                                                name: prop,
                                                value: PropertyValue::String(value),
                                            },
                                            _ => return None,
                                        }
                                    } else {
                                        Property {
                                            name: prop,
                                            value: PropertyValue::Bool(true),
                                        }
                                    });
                                }
                                Token::Slash => {
                                    if tokens.expect(Token::CloseAngle) {
                                        return Some(TagNode::VoidTag {
                                            name: ident,
                                            properties,
                                        });
                                    } else {
                                        return None;
                                    }
                                }
                                Token::CloseAngle => {}
                                _ => return None,
                            }
                        }
                        Some(TagNode::OpenTag {
                            name: ident,
                            properties,
                        })
                    }
                    _ => None,
                }
            }
            Token::Text(mut text) => {
                let mut tokens = self
                    .tokens
                    .read_while(|tok| tok != Token::OpenAngle)
                    .remove_whitespaces();
                while let Some(token) = tokens.advance() {
                    match token {
                        Token::Text(s) => {
                            text.push(' ');
                            text.push_str(&s)
                        }
                        _ => return None,
                    }
                }
                Some(TagNode::Text(text))
            }
            _ => None,
        }
    }
}
