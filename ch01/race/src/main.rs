use std::thread;

fn main() {
  let mut data: i32 = 100;

  thread::spawn(|| { data = 500; });
  thread::spawn(|| { data = 1000; });
  println!("{}", data);
}
