

use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

pub enum StatementNode {
  Let(LetStatement),
}

impl Node for StatementNode {
  fn token_literal(&self) -> String {
    match self {
      Self::Let(stmt) => stmt.token_literal(),
    }
  }
  fn print_string(&self) -> String {
    match self {
      Self::Let(stmt) => stmt.print_string(),
    }
  }
}

pub enum ExpressionNode {
  IdentifierNode(Identifier),
}

impl Node for ExpressionNode {
  fn token_literal(&self) -> String {
    match self {
      Self::IdentifierNode(identifier) => identifier.token_literal(),
    }
  }
  fn print_string(&self) -> String {
    match self {
      Self::IdentifierNode(identifier) => identifier.print_string(),
    }
  }
}

pub struct Program {
  pub statements: Vec<StatementNode>,
}

impl Node for Program {
  fn token_literal(&self) -> String {
    return if !self.statements.is_empty() { 
      match self.statements.first() {
        Some(StatementNode::Let(stmt) ) => stmt.token_literal(),
        None   => String::from("")
      }
    } else {
      String::from("")
    };
  }
  fn print_string(&self) -> String {
    let mut out = String::from("");

    for stme in self.statements.as_slice() {
      out.push_str(&stme.print_string());
    }

    out
  }
}

pub struct LetStatement {
  token: Token, 
  pub name: Identifier,
  value: Option<ExpressionNode>,
}

pub struct Identifier {
  token: Token,
  pub value: String
}

impl Node for LetStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn print_string(&self) -> String {
    let mut out = String::from("");
    out.push_str(&self.token.literal);
    out.push_str(" ");
    out.push_str(self.name.print_string().as_str());
    out.push_str(" = ");

    if let Some(value) = &self.value {
      out.push_str(value.print_string().as_str());
    } 

    out.push_str(";");

    out

  }
}

impl Node for Identifier {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn print_string(&self) -> String {
    self.value.clone()
  }
}
    