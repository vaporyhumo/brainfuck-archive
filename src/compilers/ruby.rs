use crate::Node;

pub fn compile(nodes: Vec<Node>) -> String {
  let mut code = String::new();
  code.push_str("memory = [0] * 30000\n");
  code.push_str("pointer = 0\n");

  compile_helper(&mut code, nodes)
}

pub fn compile_helper(code: &mut String, nodes: Vec<Node>) -> String {
  for node in nodes {
    match node {
      Node::Comma => code.push_str("memory[pointer] = STDIN.getc\n"),
      Node::Dot => code.push_str("print memory[pointer].chr\n"),
      Node::Left => code.push_str("pointer -= 1\n"),
      Node::Right => code.push_str("pointer += 1\n"),
      Node::Plus => code.push_str("memory[pointer] += 1\n"),
      Node::Minus => code.push_str("memory[pointer] -= 1\n"),
      Node::Loop(loop_nodes) => {
        code.push_str("memory[pointer].times do\n");
        code.push_str(&compile(loop_nodes));
        code.push_str("end\n");
      },
    }
  }
  code.to_owned()
}
