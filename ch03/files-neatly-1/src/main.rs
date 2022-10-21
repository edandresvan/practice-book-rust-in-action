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
}

fn main() {
  let f3: File = File::new("f3.txt");

  let f3_name: &String = &f3.name;
  let f3_length: usize = f3.data.len();

  println!("{:?}", f3);
  println!("{} is {} bytes long", f3_name, f3_length);
}
