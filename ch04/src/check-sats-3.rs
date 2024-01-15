#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
  id: u64,
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

fn check_status(satellite: CubeSat) -> CubeSat {
  println!("{:?}: {:?}", satellite, StatusMessage::Ok);
  satellite
}

fn main() {
  let sat_a: CubeSat = CubeSat { id: 0 };
  let sat_b: CubeSat = CubeSat { id: 1 };
  let sat_c: CubeSat = CubeSat { id: 2 };

  let sat_a: CubeSat = check_status(sat_a);
  let sat_b: CubeSat = check_status(sat_b);
  let sat_c: CubeSat = check_status(sat_c);

  let sat_a: CubeSat = check_status(sat_a);
  let sat_b: CubeSat = check_status(sat_b);
  let sat_c: CubeSat = check_status(sat_c);
}
