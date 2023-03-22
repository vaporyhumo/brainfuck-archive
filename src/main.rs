mod lexer;
use lexer::{lex, BalancedTokens, Token};

mod parser;
use parser::parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
  Plus,
  Minus,
  Right,
  Left,
  Loop(Vec<Node>),
  Dot,
  Comma,
}

fn main() {
  let code: &str = "+++++[-]";
  let tokens: BalancedTokens = lex(code);
  let nodes: Vec<Node> = parse(tokens.tokens);
  println!("{:?}", nodes);
}
