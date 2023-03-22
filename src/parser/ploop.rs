use crate::{lexer::Token, Node};

pub fn parse_loop(tokens: Vec<Token>) -> Option<(Node, Vec<Token>)> {
  let (loop_tokens, rest_tokens) = split_tokens(tokens);
  let loop_node: Node = Node::Loop(super::parse(loop_tokens));
  Some((loop_node, rest_tokens))
}

fn split_tokens(tokens: Vec<Token>) -> (Vec<Token>, Vec<Token>) {
  let head = tokens.first().unwrap();
  let mut depth: usize = 1;
  let mut loop_tokens: Vec<Token> = vec![];
  let mut close_index: usize = 0;

  if head != &Token::OpenBracket {
    panic!("Expected open bracket");
  }

  'asdf: for (index, token) in tokens[1..].iter().enumerate() {
    match token {
      Token::OpenBracket => depth += 1,
      Token::CloseBracket(_) => depth -= 1,
      _ => {}
    }

    if depth == 0 {
      close_index = index;
      break 'asdf;
    }

    loop_tokens.push(token.clone());
  }

  let rest_tokens: Vec<Token> = tokens[close_index + 2..].to_vec();

  (loop_tokens, rest_tokens)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_split_tokens() {
    assert_eq!(
      split_tokens(vec![Token::OpenBracket, Token::CloseBracket(1)]),
      (vec![], vec![])
    );

    assert_eq!(
      split_tokens(vec![
        Token::OpenBracket,
        Token::Plus,
        Token::CloseBracket(1)
      ]),
      (vec![Token::Plus], vec![])
    );

    assert_eq!(
      split_tokens(vec![
        Token::OpenBracket,
        Token::Plus,
        Token::Minus,
        Token::CloseBracket(1)
      ]),
      (vec![Token::Plus, Token::Minus], vec![])
    );

    assert_eq!(
      split_tokens(vec![
        Token::OpenBracket,
        Token::Plus,
        Token::CloseBracket(1),
        Token::Minus,
      ]),
      (vec![Token::Plus], vec![Token::Minus])
    );
  }
}
