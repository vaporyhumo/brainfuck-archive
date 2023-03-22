use crate::Token;

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

pub fn string_to_token_vec(code: &str) -> Vec<Token> {
  code.chars().enumerate().filter_map(char_to_token).collect()
}

// tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_to_token_vec() {
    let code: &str = "++>++<++[>++<++-]";
    let tokens: Vec<Token> = string_to_token_vec(code);
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