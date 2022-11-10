fn main() {
  let mut i: u16 = 0;
  print!("{index}..", index = i);

  loop {
    i += 1000;
    print!("{index}..", index = i);
    if i % 10_000 == 0 {
      println!();
    }
  }
}
