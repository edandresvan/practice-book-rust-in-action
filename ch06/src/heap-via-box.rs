use std::mem::drop;

fn main() {
  let a: Box<i32> = Box::new(1);
  let b: Box<i32> = Box::new(1);
  let c: Box<i32> = Box::new(1);

  let result_1: i32 = *a + *b + *c;

  drop(a);

  let d: Box<i32> = Box::new(1);
  let result_2: i32 = *b + *c + *d;


  println!("result_1 = {result_1}");
  println!("result_2 = {result_2}");
}
