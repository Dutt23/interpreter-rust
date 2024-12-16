use crate::token::Token;

struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lex = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 1,
            ch: Default::default(),
        };

        lex.read_char();
        
        lex
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
          self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.position += 1;
    }

    fn next_token(&self) -> Token {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::token::{Token, TokenKind};

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Lparen,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Rparen,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Lbrace,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Rbrace,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Eof,
                literal: "=".to_string(),
            },
        ];

        let lexer = Lexer::new(input);
        for (idx, exp_token) in expected.into_iter().enumerate() {
            let recv_token = lexer.next_token();
            assert_eq!(
                exp_token.kind, recv_token.kind,
                "tests({idx}) kind not equal , expected = {} but got actual = {}",
                exp_token.kind, recv_token.kind
            );
            assert_eq!(
                exp_token.literal, recv_token.literal,
                "tests({idx}) literal not equal , expected = {} but got actual = {}",
                exp_token.literal, recv_token.literal
            );
        }
    }
}
