use crate::{Token, Node};

pub(super) fn token_to_node(token: Token) -> Node {
  match token {
    Token::Plus => Node::Plus,
    Token::Minus => Node::Minus,
    Token::Right => Node::Right,
    Token::Left => Node::Left,
    Token::Dot => Node::Dot,
    Token::Comma => Node::Comma,
    _ => panic!("Unexpected token: {:?}", token),
  }
}

pub(super) fn parse_instruction(tokens: Vec<Token>) -> Option<(Node, Vec<Token>)> {
  let head: Token = tokens.get(0)?.to_owned();
  let tail: Vec<Token> = tokens[1..].to_vec();

  match head {
    Token::Plus | Token::Minus | Token::Right | Token::Left | Token::Dot | Token::Comma => {
      Some((token_to_node(head), tail))
    }
    _ => None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_token_to_node() {
    assert_eq!(token_to_node(Token::Plus), Node::Plus);
    assert_eq!(token_to_node(Token::Minus), Node::Minus);
    assert_eq!(token_to_node(Token::Right), Node::Right);
    assert_eq!(token_to_node(Token::Left), Node::Left);
    assert_eq!(token_to_node(Token::Dot), Node::Dot);
    assert_eq!(token_to_node(Token::Comma), Node::Comma);
  }

  #[test]
  #[should_panic]
  fn test_token_to_node_panic() {
    token_to_node(Token::OpenBracket);
  }

  #[test]
  #[should_panic]
  fn test_token_to_node_panic_close_bracket() {
    token_to_node(Token::CloseBracket(0));
  }

  #[test]
  fn test_parse_one_token() {
    assert_eq!(parse_instruction(vec![]), None);
    assert_eq!(parse_instruction(vec![Token::OpenBracket]), None);
    assert_eq!(parse_instruction(vec![Token::CloseBracket(1)]), None);
    assert_eq!(
      parse_instruction(vec![Token::Plus]),
      Some((Node::Plus, vec![]))
    );
    assert_eq!(
      parse_instruction(vec![Token::Plus, Token::Minus]),
      Some((Node::Plus, vec![Token::Minus]))
    );
  }
}
