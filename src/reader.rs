use std::{
  io::{self, Error, ErrorKind, Read},
  str::from_utf8,
};

pub fn read_char_from_stdin() -> char {
  let mut buffer = [0; 4];
  let mut stdin = io::stdin();

  stdin.read_exact(&mut buffer[0..1]).unwrap();
  let mut len = 1;
  if let Err(_) = from_utf8(&buffer[0..len]) {
    while len < 4 && (buffer[len - 1] & 0b11000000) == 0b10000000 {
      stdin.read_exact(&mut buffer[len..len + 1]).unwrap();
      len += 1;
    }
  }

  from_utf8(&buffer[0..len])
    .unwrap()
    .chars()
    .next()
    .ok_or(Error::new(ErrorKind::UnexpectedEof, "No character found"))
    .unwrap()
}
