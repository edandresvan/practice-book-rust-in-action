
fn main() {
  let a: f32 = 1.0;

  let frankentype: u32 = unsafe { std::mem::transmute(a) };

  println!("{}", frankentype);
  println!("{:032b}", frankentype);

  let b: f32 = unsafe { std::mem::transmute(frankentype) };

  println!("{:1.}", b);
  assert_eq!(a, b);
}
