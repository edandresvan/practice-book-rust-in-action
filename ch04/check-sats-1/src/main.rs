#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

fn check_status(satellite_id: u64) -> StatusMessage {
  StatusMessage::Ok
}

fn main() {
  let sat_a: u64 = 0;
  let sat_b: u64 = 1;
  let sat_c: u64 = 2;

  let a_status: StatusMessage = check_status(sat_a);
  let b_status: StatusMessage = check_status(sat_b);
  let c_status: StatusMessage = check_status(sat_c);

  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
