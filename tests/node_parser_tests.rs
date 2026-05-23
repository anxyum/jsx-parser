use jsx_compiler::parser::{Node, NodeParser};
use jsx_compiler::parser::{Property, PropertyValue, TagNode};

fn parse(tags: Vec<TagNode>) -> Vec<Node> {
    NodeParser::new(tags).parse()
}

#[test]
fn parses_text_node() {
    let tags = vec![TagNode::Text("hello".into())];

    let nodes = parse(tags);

    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        Node::Text(text) => {
            assert_eq!(text, "hello");
        }
        _ => panic!("Expected Text"),
    }
}

#[test]
fn parses_void_element() {
    let tags = vec![TagNode::VoidTag {
        name: "img".into(),
        properties: vec![],
    }];

    let nodes = parse(tags);

    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        Node::VoidElement { tag, properties } => {
            assert_eq!(tag, "img");
            assert!(properties.is_empty());
        }
        _ => panic!("Expected VoidElement"),
    }
}

#[test]
fn parses_simple_element() {
    let tags = vec![
        TagNode::OpenTag {
            name: "div".into(),
            properties: vec![],
        },
        TagNode::Text("hello".into()),
        TagNode::CloseTag("div".into()),
    ];

    let nodes = parse(tags);

    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        Node::Element {
            tag,
            children,
            properties,
        } => {
            assert_eq!(tag, "div");
            assert!(properties.is_empty());

            assert_eq!(children.len(), 1);

            match &children[0] {
                Node::Text(text) => {
                    assert_eq!(text, "hello");
                }
                _ => panic!("Expected text child"),
            }
        }
        _ => panic!("Expected Element"),
    }
}

#[test]
fn parses_nested_elements() {
    let tags = vec![
        TagNode::OpenTag {
            name: "div".into(),
            properties: vec![],
        },
        TagNode::OpenTag {
            name: "span".into(),
            properties: vec![],
        },
        TagNode::Text("test".into()),
        TagNode::CloseTag("span".into()),
        TagNode::CloseTag("div".into()),
    ];

    let nodes = parse(tags);

    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        Node::Element { tag, children, .. } => {
            assert_eq!(tag, "div");
            assert_eq!(children.len(), 1);

            match &children[0] {
                Node::Element { tag, children, .. } => {
                    assert_eq!(tag, "span");
                    assert_eq!(children.len(), 1);

                    match &children[0] {
                        Node::Text(text) => {
                            assert_eq!(text, "test");
                        }
                        _ => panic!("Expected text"),
                    }
                }

                _ => panic!("Expected span"),
            }
        }

        _ => panic!("Expected div"),
    }
}

#[test]
fn parses_properties() {
    let tags = vec![
        TagNode::OpenTag {
            name: "div".into(),
            properties: vec![
                Property {
                    name: "class".into(),
                    value: PropertyValue::String("container".into()),
                },
                Property {
                    name: "hidden".into(),
                    value: PropertyValue::Bool(true),
                },
            ],
        },
        TagNode::CloseTag("div".into()),
    ];

    let nodes = parse(tags);

    match &nodes[0] {
        Node::Element { properties, .. } => {
            assert_eq!(properties.len(), 2);

            assert_eq!(properties[0].name, "class");

            match &properties[0].value {
                PropertyValue::String(v) => {
                    assert_eq!(v, "container");
                }
                _ => panic!("Expected string"),
            }

            assert_eq!(properties[1].name, "hidden");

            match properties[1].value {
                PropertyValue::Bool(v) => {
                    assert!(v);
                }
                _ => panic!("Expected bool"),
            }
        }

        _ => panic!("Expected element"),
    }
}

#[test]
fn parses_multiple_root_nodes() {
    let tags = vec![
        TagNode::Text("hello".into()),
        TagNode::VoidTag {
            name: "br".into(),
            properties: vec![],
        },
        TagNode::Text("world".into()),
    ];

    let nodes = parse(tags);

    assert_eq!(nodes.len(), 3);

    match &nodes[0] {
        Node::Text(text) => assert_eq!(text, "hello"),
        _ => panic!("Expected text"),
    }

    match &nodes[1] {
        Node::VoidElement { tag, .. } => {
            assert_eq!(tag, "br");
        }
        _ => panic!("Expected br"),
    }

    match &nodes[2] {
        Node::Text(text) => assert_eq!(text, "world"),
        _ => panic!("Expected text"),
    }
}
