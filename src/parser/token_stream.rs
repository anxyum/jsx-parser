use crate::lexer::Token;

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn finished(&self) -> bool {
        self.tokens.len() == self.pos
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn advance(&mut self) -> Option<Token> {
        let tok = self.peek()?.clone();
        self.pos += 1;
        Some(tok)
    }

    pub fn expect(&mut self, expected: Token) -> bool {
        if let Some(tok) = self.peek() {
            if *tok == expected {
                self.pos += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn skip_whitespace(&mut self) {
        let token = self.peek();
        if let Some(token) = token
            && *token == Token::WhiteSpace
        {
            self.pos += 1;
        }
    }

    pub fn read_while<F>(&mut self, cond: F) -> TokenStream
    where
        F: Fn(Token) -> bool,
    {
        let mut tokens = vec![];

        while let Some(token) = self.peek() {
            if !cond(token.clone()) {
                break;
            }
            tokens.push(token.clone());
            self.advance();
        }

        TokenStream::new(tokens)
    }

    pub fn read_until<F>(&mut self, cond: F) -> TokenStream
    where
        F: Fn(Token) -> bool,
    {
        let mut tokens = vec![];

        while let Some(token) = self.peek() {
            let token = token.clone();
            tokens.push(token.clone());
            self.pos += 1;
            if cond(token) {
                break;
            }
        }

        TokenStream::new(tokens)
    }

    pub fn remove_whitespaces(mut self) -> TokenStream {
        let mut tokens = vec![];
        self.pos = 0;
        self.skip_whitespace();
        while let Some(tok) = self.advance() {
            tokens.push(tok);
            self.skip_whitespace();
        }
        Self::new(tokens)
    }
}
