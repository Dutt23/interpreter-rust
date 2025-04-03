use std::{fmt::write, str::FromStr};

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug, Default, Clone)]
pub enum TokenKind {
    #[default]
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Eq,
    NotEq,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl TokenKind {
    pub fn lookup_ident(str: &str) -> TokenKind {
        match str {
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Ident,
        }
    }

    pub fn to_literal(&self, literal: char) -> String {
        if &Self::Eq == self {
            return "==".to_string();
        } else if &Self::NotEq == self {
            return "!=".to_string();
        }
        literal.to_string()
    }

    pub fn to_tok(s: &str, next_char: &str) -> Self {
        match s {
            "=" => {
                if next_char == "=" {
                    return self::TokenKind::Eq;
                }
                self::TokenKind::Assign
            }
            "+" => self::TokenKind::Plus,
            "(" => self::TokenKind::Lparen,
            ")" => self::TokenKind::Rparen,
            "{" => self::TokenKind::Lbrace,
            "}" => self::TokenKind::Rbrace,
            "fn" => self::TokenKind::Function,
            "let" => self::TokenKind::Let,
            "if" => self::TokenKind::If,
            "else" => self::TokenKind::Else,
            "return" => self::TokenKind::Return,
            "," => self::TokenKind::Comma,
            ";" => self::TokenKind::Semicolon,
            "\0" => self::TokenKind::Eof,
            "/" => self::TokenKind::Slash,
            "-" => self::TokenKind::Minus,
            "!" => {
                if next_char == "=" {
                    return self::TokenKind::NotEq;
                }
                self::TokenKind::Bang
            }
            "*" => self::TokenKind::Asterisk,
            "<" => self::TokenKind::Lt,
            ">" => self::TokenKind::Gt,
            _ => self::TokenKind::Illegal,
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for TokenKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(self::TokenKind::Assign),
            "==" => Ok(self::TokenKind::Eq),
            "!=" => Ok(self::TokenKind::NotEq),
            "+" => Ok(self::TokenKind::Plus),
            "(" => Ok(self::TokenKind::Lparen),
            ")" => Ok(self::TokenKind::Rparen),
            "{" => Ok(self::TokenKind::Lbrace),
            "}" => Ok(self::TokenKind::Rbrace),
            "fn" => Ok(self::TokenKind::Function),
            "let" => Ok(self::TokenKind::Let),
            "if" => Ok(self::TokenKind::If),
            "else" => Ok(self::TokenKind::Else),
            "return" => Ok(self::TokenKind::Return),
            "," => Ok(self::TokenKind::Comma),
            ";" => Ok(self::TokenKind::Semicolon),
            "\0" => Ok(self::TokenKind::Eof),
            "/" => Ok(self::TokenKind::Slash),
            "-" => Ok(self::TokenKind::Minus),
            "!" => Ok(self::TokenKind::Bang),
            "*" => Ok(self::TokenKind::Asterisk),
            "<" => Ok(self::TokenKind::Lt),
            ">" => Ok(self::TokenKind::Gt),
            _ => Ok(self::TokenKind::Illegal),
        }
    }
}
