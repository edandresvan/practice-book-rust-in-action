#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
  id: u64,
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

fn check_status(satellite: CubeSat) -> StatusMessage {
  StatusMessage::Ok
}

fn main() {
  let sat_a: CubeSat = CubeSat { id: 0 };
  let sat_b: CubeSat = CubeSat { id: 1 };
  let sat_c: CubeSat = CubeSat { id: 2 };

  let a_status: StatusMessage = check_status(sat_a);
  let b_status: StatusMessage = check_status(sat_b);
  let c_status: StatusMessage = check_status(sat_c);

  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

/*   let a_status: StatusMessage = check_status(sat_a);
  let b_status: StatusMessage = check_status(sat_b);
  let c_status: StatusMessage = check_status(sat_c); */

  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
