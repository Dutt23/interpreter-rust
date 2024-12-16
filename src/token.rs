use std::{fmt::write, str::FromStr};

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
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
            "+" => Ok(self::TokenKind::Plus),
            "(" => Ok(self::TokenKind::Lparen),
            ")" => Ok(self::TokenKind::Rparen),
            "{" => Ok(self::TokenKind::Lbrace),
            "}" => Ok(self::TokenKind::Rbrace),
            "fn" => Ok(self::TokenKind::Function),
            "let" => Ok(self::TokenKind::Let),
            "," => Ok(self::TokenKind::Comma),
            ";" => Ok(self::TokenKind::Semicolon),
            "\0" => Ok(self::TokenKind::Eof),
            _ => Ok(self::TokenKind::Illegal),
        }
    }
}
