use crate::{lexer::Lexer, token::Token};

struct Parser {
  lexer: Lexer,
  curr_token: Token,
  peek_token: Token,
}

impl Parser {

   pub fn new(lexer: Lexer) -> Self {
   let mut parser = Self {
      lexer: lexer,
      curr_token: Default::default(),
      peek_token: Default::default(),
    };

    parser.next_token();
    parser.next_token();
    
    parser
   }

   fn next_token(&mut self){ 
    self.curr_token = self.peek_token.clone();
    self.peek_token = self.lexer.next_token();
   }
}