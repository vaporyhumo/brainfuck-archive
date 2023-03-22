use crate::{reader, Node, VM};

pub fn run(nodes: Vec<Node>) {
  let mut vm: VM = VM {
    memory: [0; 30000],
    pointer: 0,
    out: String::new(),
  };
  nodes.iter().fold(vm, |vm, node| run_node(vm, node.clone()));
}

pub fn run_node(vm: VM, node: Node) -> VM {
  let mut vm: VM = vm;

  match node {
    Node::Left => {
      vm.pointer -= 1;
    }
    Node::Right => {
      vm.pointer += 1;
    }
    Node::Plus => {
      vm.memory[vm.pointer] = vm.memory[vm.pointer].wrapping_add(1);
    }
    Node::Minus => {
      vm.memory[vm.pointer] = vm.memory[vm.pointer].wrapping_sub(1);
    }
    Node::Dot => {
      vm.out.push(vm.memory[vm.pointer] as char);
      print!("{}", vm.memory[vm.pointer] as char);
    }
    Node::Comma => {
      let car: u8 = reader::read_char_from_stdin() as u8;
      vm.memory[vm.pointer] = car;
    }
    Node::Loop(nodes) => {
      while vm.memory[vm.pointer] != 0 {
        vm = nodes.iter().fold(vm, |vm, node| run_node(vm, node.clone()));
      }
    }
  }
  //debug_vm(&vm);
  vm
}

fn debug_vm(vm: &VM) {
  for (i, &x) in vm.memory.iter().enumerate() {
    if x != 0 {
      println!("Index: {}, Value: {} {}", i, x, x as char);
    }
  }
}
