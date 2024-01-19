fn main() {
  let a: i64 = 42;
  let a_pointer: *const i64  = &a as *const i64;

  let a_address: usize = unsafe {
    std::mem::transmute(a_pointer)
  };

   println!("a: {} ({:p} ... 0x{:x})", a, a_pointer, a_address + 7);
}
