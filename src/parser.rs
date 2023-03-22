use crate::{Node, Token};

mod instruction;

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
  //let maybe_first_token: Option<&Token> = tokens.get(0);

  //match maybe_first_token {
  //  Some(token) => match token {
  //    Token::Comma | Token::Dot | Token::Left | Token::Right | Token::Minus | Token::Plus => {
  //      let (node, rest): (Node, Vec<Token>) = parse_instruction(tokens).unwrap();
  //      let mut nodes: Vec<Node> = vec![node];
  //      nodes.append(&mut token_vec_to_node_vec(rest));
  //      nodes
  //    }
  //    Token::OpenBracket => {
  //      let (node, rest): (Node, Vec<Token>) = parse_loop(tokens).unwrap();
  //      let mut nodes: Vec<Node> = vec![node];
  //      nodes.append(&mut token_vec_to_node_vec(rest));
  //      nodes
  //    }
  //    t => panic!("Unexpected token: {:?}", t),
  //  },
  //  None => vec![],
  //}
  vec![]
}

// tests
#[cfg(test)]
mod tests {
}
