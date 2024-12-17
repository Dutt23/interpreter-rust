use std::str::FromStr;

use crate::token::{Token, TokenKind};

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
            read_position: 0,
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
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = Token {
            kind: TokenKind::from_str(&self.ch.to_string()).unwrap(),
            literal: self.ch.to_string(),
        };

        // Clean this later
        if tok.kind == TokenKind::Illegal && Self::is_letter(self.ch) {
            let literal = self.read_identifier();
            let kind = TokenKind::lookup_ident(&literal);
            return Token { kind, literal };
        }

        if tok.kind == TokenKind::Illegal && Self::is_num(self.ch) {
            let literal = self.read_num();
            let kind = TokenKind::Int;
            return Token { kind, literal };
        }
        self.read_char();

        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_num(ch: char) -> bool {
        ch.is_numeric()
    }

    fn read_num(&mut self) -> String {
        let mut num = String::new();

        while Self::is_num(self.ch) {
            num.push(self.ch);
            self.read_char();
        }

        num
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while Self::is_letter(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }

        return ident;
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
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Lparen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Rparen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Eof,
                literal: "\0".to_string(),
            },
        ];
        compare(expected, input);
    }

    #[test]
    fn test_mn_lang() {
        let input = r#"
      let five = 5;
      let ten = 10;
      
      let add = fn (x , y) {
        x + y;
      }

      let result = add(five, ten);
      "#;

        let mut five_ident = get_ident("5", "five");
        let mut ten_ident = get_ident("10", "ten");
        five_ident.append(&mut ten_ident);
        let mut expected = vec![
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Function,
                literal: "fn".to_string(),
            },
            Token {
                kind: TokenKind::Lparen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::Rparen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "result".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::Lparen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::Rparen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
        ];
        five_ident.append(&mut expected);
        compare(five_ident, input);
    }

    fn compare(expected: Vec<Token>, input: &str) {
        let mut lexer = Lexer::new(input);
        let mut count = 0;
        let count_exp = expected.len();
        for (idx, exp_token) in expected.into_iter().enumerate() {
          count += 1;
            let recv_token = lexer.next_token();
            assert_eq!(
                exp_token.kind, recv_token.kind,
                "Wrong kind given at idx {} for kind expected {:?} and actual {:?}",
                idx, exp_token.kind, recv_token.kind
            );
            assert_eq!(
                exp_token.literal, recv_token.literal,
                "Wrong literal given at idx {} for literal expected {:?} and actual {:?}",
                idx, exp_token.literal, recv_token.literal
            );
        }
        assert_eq!(count_exp, count, "Count of tokens not equal");
    }

    fn get_ident(ident: &str, val: &str) -> Vec<Token> {
        return vec![
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: val.to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: ident.to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
        ];
    }
}
