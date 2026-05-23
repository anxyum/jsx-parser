use jsx_compiler::lexer::{Lexer, Token};
use jsx_compiler::parser::{Property, PropertyValue, TagNode, TagParser};

fn parse_tokens(tokens: Vec<Token>) -> Vec<TagNode> {
    TagParser::new(tokens).parse()
}

fn parse(input: &str) -> Vec<TagNode> {
    let tokens = Lexer::new(input).tokenize();
    TagParser::new(tokens).parse()
}

#[test]
fn parses_simple_open_tag() {
    let tokens = vec![
        Token::OpenAngle,
        Token::Identifier("div".into()),
        Token::CloseAngle,
    ];

    let nodes = parse_tokens(tokens);

    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        TagNode::OpenTag { name, properties } => {
            assert_eq!(name, "div");
            assert!(properties.is_empty());
        }
        _ => panic!("Expected OpenTag"),
    }
}

#[test]
fn parses_close_tag() {
    let tokens = vec![
        Token::OpenAngle,
        Token::Slash,
        Token::Identifier("div".into()),
        Token::CloseAngle,
    ];

    let nodes = parse_tokens(tokens);

    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        TagNode::CloseTag(name) => {
            assert_eq!(name, "div");
        }
        _ => panic!("Expected CloseTag"),
    }
}

#[test]
fn parses_void_tag() {
    let tokens = vec![
        Token::OpenAngle,
        Token::Identifier("img".into()),
        Token::Slash,
        Token::CloseAngle,
    ];

    let nodes = parse_tokens(tokens);

    match &nodes[0] {
        TagNode::VoidTag { name, properties } => {
            assert_eq!(name, "img");
            assert!(properties.is_empty());
        }
        _ => panic!("Expected VoidTag"),
    }
}

#[test]
fn parses_text_node() {
    let tokens = vec![Token::Text("hello".into())];

    let nodes = parse_tokens(tokens);

    match &nodes[0] {
        TagNode::Text(text) => {
            assert_eq!(text, "hello");
        }
        _ => panic!("Expected Text"),
    }
}

#[test]
fn parses_tag_with_string_property() {
    let tokens = vec![
        Token::OpenAngle,
        Token::Identifier("div".into()),
        Token::Identifier("class".into()),
        Token::Equals,
        Token::String("container".into()),
        Token::CloseAngle,
    ];

    let nodes = parse_tokens(tokens);

    match &nodes[0] {
        TagNode::OpenTag { name, properties } => {
            assert_eq!(name, "div");
            assert_eq!(properties.len(), 1);

            let prop = &properties[0];

            assert_eq!(prop.name, "class");

            match &prop.value {
                PropertyValue::String(v) => {
                    assert_eq!(v, "container");
                }
                _ => panic!("Expected string property"),
            }
        }
        _ => panic!("Expected OpenTag"),
    }
}

#[test]
fn parses_boolean_property() {
    let tokens = vec![
        Token::OpenAngle,
        Token::Identifier("input".into()),
        Token::Identifier("disabled".into()),
        Token::CloseAngle,
    ];

    let nodes = parse_tokens(tokens);

    match &nodes[0] {
        TagNode::OpenTag { properties, .. } => {
            assert_eq!(properties.len(), 1);

            let prop = &properties[0];

            assert_eq!(prop.name, "disabled");

            match prop.value {
                PropertyValue::Bool(v) => {
                    assert!(v);
                }
                _ => panic!("Expected bool property"),
            }
        }
        _ => panic!("Expected OpenTag"),
    }
}

#[test]
fn lexer_and_parser_simple_html() {
    let nodes = parse("<div>Hello World</div>");

    assert_eq!(nodes.len(), 3);

    match &nodes[0] {
        TagNode::OpenTag { name, .. } => {
            assert_eq!(name, "div");
        }
        _ => panic!("Expected OpenTag"),
    }

    match &nodes[1] {
        TagNode::Text(text) => {
            assert_eq!(text, "Hello World");
        }
        _ => panic!("Expected Text"),
    }

    match &nodes[2] {
        TagNode::CloseTag(name) => {
            assert_eq!(name, "div");
        }
        _ => panic!("Expected CloseTag"),
    }
}

#[test]
fn lexer_and_parser_nested_html() {
    let nodes = parse("<div><span>test</span></div>");

    assert_eq!(nodes.len(), 5);

    match &nodes[0] {
        TagNode::OpenTag { name, .. } => {
            assert_eq!(name, "div");
        }
        _ => panic!("Expected div"),
    }

    match &nodes[1] {
        TagNode::OpenTag { name, .. } => {
            assert_eq!(name, "span");
        }
        _ => panic!("Expected span"),
    }

    match &nodes[2] {
        TagNode::Text(text) => {
            assert_eq!(text, "test");
        }
        _ => panic!("Expected text"),
    }

    match &nodes[3] {
        TagNode::CloseTag(name) => {
            assert_eq!(name, "span");
        }
        _ => panic!("Expected closing span"),
    }

    match &nodes[4] {
        TagNode::CloseTag(name) => {
            assert_eq!(name, "div");
        }
        _ => panic!("Expected closing div"),
    }
}

#[test]
fn lexer_and_parser_voidtag() {
    let nodes = parse("<img href=\"http://youtube.com\" />");

    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        TagNode::VoidTag { name, properties } => {
            assert_eq!(name, "img");
            assert_eq!(properties.len(), 1);
            assert_eq!(
                properties[0],
                Property {
                    name: "href".into(),
                    value: PropertyValue::String("http://youtube.com".into())
                }
            );
        }
        _ => panic!("Expected VoidTag"),
    }
}
