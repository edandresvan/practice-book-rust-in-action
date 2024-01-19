static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
  let a: usize = 42;
  let b: &[u8; 10] = &B;
  let c: Box<[u8]> = Box::new(C);

  println!("a: {}", a);
  println!("b: {:p}", b);
  println!("c: {:p}", c);
}
