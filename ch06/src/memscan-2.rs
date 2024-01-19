fn main() {
  let mut number_nonzero: i32 = 0;

  for i in 1..10_000 {
    println!("i: {}", i);
    
    let pointer: *const u8 = i as *const u8;
    let byte_at_address: u8 = unsafe { *pointer };

    if byte_at_address != 0 {
      number_nonzero += 1;
    }
  }

  println!("non-zero bytes in memory: {}", number_nonzero);
}
