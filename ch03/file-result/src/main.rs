use std::borrow::Cow;

use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
  thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
}

impl File {
  fn new(name: &str) -> Self {
    Self {
      name: String::from(name),
      data: Vec::new(),
    }
  }

  fn new_with_data(
    name: &str,
    data: &Vec<u8>,
  ) -> Self {
    let mut f = Self::new(name);
    f.data = data.clone();
    f
  }

  fn read(
    self: &Self,
    save_to: &mut Vec<u8>,
  ) -> Result<usize, String> {
    let mut tmp: Vec<u8> = self.data.clone();
    let read_length: usize = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)
  }
}

fn open(f: File) -> Result<File, String> {
  if one_in(10_000) {
    let err_msg: String = String::from("Permission denied");
    return Err(err_msg);
  }
  Ok(f)
}

fn close(f: File) -> Result<File, String> {
  if one_in(10_000) {
    let err_msg: String = String::from("Interrupted by a signal");
    return Err(err_msg);
  }
  Ok(f)
}

fn main() {
  let f4_data: Vec<u8> = vec![
    82, 117, 115, 116, 32, 105, 115, 32, 97, 110, 32, 97, 119, 101, 115, 111, 109, 101,
    32, 112, 114, 111, 103, 114, 97, 109, 109, 105, 110, 103, 32, 108, 97, 110, 103, 117,
    97, 103, 101, 33,
  ];
  let mut f4: File = File::new_with_data("4.txt", &f4_data);

  let mut buffer: Vec<u8> = vec![];

  f4 = open(f4).unwrap();
  let f4_length: usize = f4.read(&mut buffer).unwrap();
  f4 = close(f4).unwrap();

  let text: Cow<str> = String::from_utf8_lossy(&buffer);

  println!("{:?}", f4);
  println!("{} is {} bytes long", &f4.name, f4_length);
  println!("{}", text);
}
