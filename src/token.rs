struct Token {
  kind: TokenKind,
  literal: String,
}

enum TokenKind {
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