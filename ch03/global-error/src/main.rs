use std::vec;

use rand::random;

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(
  f: &File,
  save_to: &mut Vec<u8>,
) -> usize {
  if random() && random() && random() {
    unsafe { ERROR = 1 }
  }
  0
}

#[allow(unused_mut)]
fn main() {
  let mut f: File = File;
  let mut buffer: Vec<u8> = vec![];

  read(&f, &mut buffer);
  unsafe {
    if ERROR != 0 {
      panic!("An error has occured!")
    }
  }
}
