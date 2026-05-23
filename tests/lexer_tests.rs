use jsx_compiler::lexer::{Lexer, Token};

fn lex(input: &str) -> Vec<Token> {
    Lexer::new(input).tokenize()
}

#[test]
fn simple_element() {
    let tokens = lex("<div></div>");

    assert_eq!(
        tokens,
        vec![
            Token::OpenAngle,
            Token::Identifier("div".into()),
            Token::CloseAngle,
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("div".into()),
            Token::CloseAngle,
        ]
    );
}

#[test]
fn text_node() {
    let tokens = lex("<div>hello</div>");

    assert_eq!(
        tokens,
        vec![
            Token::OpenAngle,
            Token::Identifier("div".into()),
            Token::CloseAngle,
            Token::Text("hello".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("div".into()),
            Token::CloseAngle,
        ]
    );
}

#[test]
fn attribute_parsing() {
    let tokens = lex("<div class=\"a\"></div>");

    assert_eq!(
        tokens,
        vec![
            Token::OpenAngle,
            Token::Identifier("div".into()),
            Token::WhiteSpace,
            Token::Identifier("class".into()),
            Token::Equals,
            Token::String("a".into()),
            Token::CloseAngle,
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("div".into()),
            Token::CloseAngle,
        ]
    );
}

#[test]
fn nested_text() {
    let tokens = lex("<div>hello world</div>");

    assert_eq!(
        tokens,
        vec![
            Token::OpenAngle,
            Token::Identifier("div".into()),
            Token::CloseAngle,
            Token::Text("hello".into()),
            Token::WhiteSpace,
            Token::Text("world".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("div".into()),
            Token::CloseAngle,
        ]
    );
}

#[test]
fn full_document() {
    let tokens = lex(r##"<html lang="fr">
        <head>
            <meta charset="UTF-8" />
            <title>Page Exemple</title>
        </head>
        <body>
            <h1>Ma petite page web</h1>

            <p>Ceci est une page HTML simple avec du contenu basique.</p>

            <ul>
                <li>Structure HTML simple</li>
                <li>Sans CSS</li>
                <li>Sans JavaScript</li>
            </ul>

            <p>Email : exemple@mail.com</p>
        </body>
        </html>"##);

    assert_eq!(
        tokens,
        vec![
            Token::OpenAngle,
            Token::Identifier("html".into()),
            Token::WhiteSpace,
            Token::Identifier("lang".into()),
            Token::Equals,
            Token::String("fr".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("head".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("meta".into()),
            Token::WhiteSpace,
            Token::Identifier("charset".into()),
            Token::Equals,
            Token::String("UTF-8".into()),
            Token::WhiteSpace,
            Token::Slash,
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("title".into()),
            Token::CloseAngle,
            Token::Text("Page".into()),
            Token::WhiteSpace,
            Token::Text("Exemple".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("title".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("head".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("body".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("h1".into()),
            Token::CloseAngle,
            Token::Text("Ma".into()),
            Token::WhiteSpace,
            Token::Text("petite".into()),
            Token::WhiteSpace,
            Token::Text("page".into()),
            Token::WhiteSpace,
            Token::Text("web".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("h1".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("p".into()),
            Token::CloseAngle,
            Token::Text("Ceci".into()),
            Token::WhiteSpace,
            Token::Text("est".into()),
            Token::WhiteSpace,
            Token::Text("une".into()),
            Token::WhiteSpace,
            Token::Text("page".into()),
            Token::WhiteSpace,
            Token::Text("HTML".into()),
            Token::WhiteSpace,
            Token::Text("simple".into()),
            Token::WhiteSpace,
            Token::Text("avec".into()),
            Token::WhiteSpace,
            Token::Text("du".into()),
            Token::WhiteSpace,
            Token::Text("contenu".into()),
            Token::WhiteSpace,
            Token::Text("basique.".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("p".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("ul".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("li".into()),
            Token::CloseAngle,
            Token::Text("Structure".into()),
            Token::WhiteSpace,
            Token::Text("HTML".into()),
            Token::WhiteSpace,
            Token::Text("simple".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("li".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("li".into()),
            Token::CloseAngle,
            Token::Text("Sans".into()),
            Token::WhiteSpace,
            Token::Text("CSS".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("li".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("li".into()),
            Token::CloseAngle,
            Token::Text("Sans".into()),
            Token::WhiteSpace,
            Token::Text("JavaScript".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("li".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("ul".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Identifier("p".into()),
            Token::CloseAngle,
            Token::Text("Email".into()),
            Token::WhiteSpace,
            Token::Text(":".into()),
            Token::WhiteSpace,
            Token::Text("exemple@mail.com".into()),
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("p".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("body".into()),
            Token::CloseAngle,
            Token::WhiteSpace,
            Token::OpenAngle,
            Token::Slash,
            Token::Identifier("html".into()),
            Token::CloseAngle,
        ]
    );
}
