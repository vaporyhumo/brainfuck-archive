use crate::{Node, Token};

mod instruction;
mod ploop;

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
  let maybe_first_token: Option<&Token> = tokens.get(0);

  match maybe_first_token {
    Some(token) => match token {
      Token::Comma | Token::Dot | Token::Left | Token::Right | Token::Minus | Token::Plus => {
        let (node, rest): (Node, Vec<Token>) = instruction::parse_instruction(tokens).unwrap();
        let mut nodes = vec![node];
        nodes.append(&mut parse(rest));
        nodes
      }
      Token::OpenBracket => {
        let (node, rest): (Node, Vec<Token>) = ploop::parse_loop(tokens).unwrap();
        let mut nodes: Vec<Node> = vec![node];
        nodes.append(&mut parse(rest));
        nodes
      }
      Token::CloseBracket(_) => {
        panic!("Unexpected close bracket");
      }
    },
    None => vec![],
  }
}

// tests
#[cfg(test)]
mod tests {}
