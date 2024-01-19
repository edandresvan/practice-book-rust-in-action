static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
  let noop_local: i32 = 12_345;
  &noop_local as *const i32
}

fn main() {
  let local_str: &str = "a";
  let local_int: i32 = 123;
  let boxed_str: Box<char> = Box::new('b');
  let boxed_int: Box<i32> = Box::new(789);
  let fn_int: *const i32 = noop();

  println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
  println!("local_str: {:p}", local_str as *const str);
  println!("local_int: {:p}", &local_int as *const i32);
  println!("boxed_int: {:p}", Box::into_raw(boxed_int));
  println!("boxed_str: {:p}", Box::into_raw(boxed_str));
  println!("fn_int:    {:p}", fn_int);
}
