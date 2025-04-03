use crate::{ast::Program, lexer::Lexer, token::Token};

pub struct Parser {
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

   pub fn parse_program(&mut self) -> Option<Program> {
    None
   }
}

#[cfg(test)]
mod test {
    use crate::{ast::{Node, StatementNode}, lexer::Lexer};

    use super::Parser;


  #[test]
  fn test_let_statements() {
    let input = r#"
    let x = 5;
    let y = 10;
    let foobar = 939393;
    "#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    match program {
      Some(prog) => {
      assert_eq!(prog.statements.len(), 3);

      let expected = vec!["x", "y", "foobar"];

      for (idx, exp) in expected.into_iter().enumerate() {
        let stmt = &prog.statements[idx];
        test_let_statement(stmt, exp);
      }
    },
    None => panic!("Expected a program but got None"),
  }
}

fn test_let_statement(stmt: &StatementNode, expected: &str) {
  assert_eq!(stmt.token_literal(), "let", "Token literal not let");
  match stmt {
    StatementNode::Let(let_stmt) => {
      assert_eq!(let_stmt.name.value, expected, "Name not {}", expected);
    },
    _ => panic!("Statement is not let statement")
  }
}

}