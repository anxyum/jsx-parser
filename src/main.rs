mod lexer;
mod parser;

use lexer::Lexer;
use parser::TagParser;

const PROJECT_FOLDER: &str = "./playground/";

fn main() {
    let mut src_path = PROJECT_FOLDER.to_string();
    src_path.push_str("src/index.html");
    let mut dist_path = PROJECT_FOLDER.to_string();
    dist_path.push_str("dist/index.html");

    let input = std::fs::read_to_string(src_path);

    if let Ok(input) = input {
        let tokens = Lexer::new(&input).tokenize();
        println!("{:#?}", tokens);
        let tag_nodes = TagParser::new(tokens).parse();
        println!("{:#?}", tag_nodes);
    } else {
        println!("file not found");
    }
}
