mod lexer;

use lexer::Lexer;

fn main() {
    let input = std::fs::read_to_string("playground/tests/single div.html");

    if let Ok(input) = input {
        let tokens = Lexer::new(&input).tokenize();
        println!("{:#?}", tokens);
    } else {
        println!("file not found");
    }
}
