use super::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    inside_tag: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            inside_tag: false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        Some(c)
    }

    fn read_while<F>(&mut self, cond: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if !cond(c) {
                break;
            }
            s.push(c);
            self.pos += 1;
        }
        s
    }

    fn skip_whitespace(&mut self) {
        self.read_while(|c| c.is_whitespace());
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let c = self.advance()?;

        match c {
            '<' => {
                self.inside_tag = true;
                Some(Token::OpenAngle)
            }

            '>' => {
                self.inside_tag = false;
                Some(Token::CloseAngle)
            }

            '/' => Some(Token::Slash),

            '=' => Some(Token::Equal),

            '"' => {
                let s = self.read_while(|c| c != '"');
                self.advance(); // consume closing "
                Some(Token::String(s))
            }

            c if self.inside_tag && c.is_alphabetic() => {
                let mut ident = c.to_string();
                ident.push_str(&self.read_while(|c| c.is_alphanumeric()));
                Some(Token::Identifier(ident))
            }

            c => {
                let mut text = c.to_string();
                text.push_str(&self.read_while(|c| c != '<'));
                Some(Token::Text(text))
            }

            _ => None,
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}
