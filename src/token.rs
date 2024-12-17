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
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

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
