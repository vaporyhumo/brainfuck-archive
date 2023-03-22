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

fn main() {
  let code: &str = "+++++[-]";
  let tokens: Vec<Token> = string_to_token_vec(code);
  println!("{:?}", tokens);
}
