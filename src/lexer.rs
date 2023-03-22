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

pub struct BalancedTokens {
  pub tokens: Vec<Token>,
}

impl BalancedTokens {
  pub fn new(tokens: Vec<Token>) -> Self {
    let mut brackets: usize = 0;

    for token in tokens.iter() {
      match token {
        Token::OpenBracket => brackets += 1,
        Token::CloseBracket(_) => brackets -= 1,
        _ => (),
      }
    }

    if brackets != 0 {
      panic!("Unbalanced brackets");
    }

    Self { tokens }
  }
}

fn char_to_token((i, c): (usize, char)) -> Option<Token> {
  match c {
    '+' => Some(Token::Plus),
    '-' => Some(Token::Minus),
    '>' => Some(Token::Right),
    '<' => Some(Token::Left),
    '[' => Some(Token::OpenBracket),
    ']' => Some(Token::CloseBracket(i)),
    '.' => Some(Token::Dot),
    ',' => Some(Token::Comma),
    _ => None,
  }
}

pub fn lex(code: &str) -> BalancedTokens {
  BalancedTokens::new(code.chars().enumerate().filter_map(char_to_token).collect())
}

// tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lex() {
    let code: &str = "++>++<++[>++<++-]";
    let tokens: Vec<Token> = lex(code).tokens;
    assert_eq!(
      tokens,
      vec![
        Token::Plus,
        Token::Plus,
        Token::Right,
        Token::Plus,
        Token::Plus,
        Token::Left,
        Token::Plus,
        Token::Plus,
        Token::OpenBracket,
        Token::Right,
        Token::Plus,
        Token::Plus,
        Token::Left,
        Token::Plus,
        Token::Plus,
        Token::Minus,
        Token::CloseBracket(16)
      ]
    );
  }
}
