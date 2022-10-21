#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
}

fn main() {
  let f1: File = File {
    name: String::from("f1.txt"),
    data: Vec::new(),
  };

  let f1_name: &String = &f1.name;
  let f1_length: &usize = &f1.data.len();

  println!("{:?}", f1);
  println!("{} is {} bytes long", f1_name, f1_length);
}
