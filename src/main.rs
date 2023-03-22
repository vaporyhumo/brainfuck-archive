mod lexer;
use lexer::string_to_token_vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
  Plus,
  Minus,
  Right,
  Left,
  OpenBracket,
  CloseBracket(usize),
  Dot,
  Comma,
}

mod parser;
use parser::token_vec_to_node_vec;

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
  let tokens: Vec<Token> = string_to_token_vec(code);
  let nodes: Vec<Node> = token_vec_to_node_vec(tokens);
  println!("{:?}", nodes);
}
