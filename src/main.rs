mod lexer;
use lexer::{lex, BalancedTokens, Token};

mod parser;
use parser::parse;

mod reader;

mod interpreter;
use interpreter::run;

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

pub struct VM {
  memory: [u8; 30000],
  pointer: usize,
  out: String,
}

fn main() {
  let code: &str = ",.,.,.";
  let tokens: BalancedTokens = lex(code);
  let nodes: Vec<Node> = parse(tokens.tokens);
  run(nodes);
}
