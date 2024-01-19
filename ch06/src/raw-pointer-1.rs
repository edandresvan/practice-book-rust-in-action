fn main() {
  let a: i64 = 42;
  let a_pointer: *const i64 = &a as *const i64;

  println!("a: {} ({:p})", a, a_pointer);
}
