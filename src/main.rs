mod lexer;
use lexer::lex;

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
  //let code: &str = "+++++[-]";
  let code: &str = "+++++";
  let tokens: Vec<Token> = lex(code);
  let nodes: Vec<Node> = parse(tokens);
  println!("{:?}", nodes);
}
