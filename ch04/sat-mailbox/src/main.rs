#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
  id: u64,
  mailbox: Mailbox,
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

type Message = String;

struct GroundStation {}

impl GroundStation {
  fn send(
    &self,
    to: &mut CubeSat,
    msg: Message,
  ) {
    to.mailbox.messages.push(msg);
  }
}

impl CubeSat {
  fn recv(&mut self) -> Option<Message> {
    self.mailbox.messages.pop()
  }
}

fn check_status(satellite: CubeSat) -> CubeSat {
  println!("{:?}: {:?}", satellite, StatusMessage::Ok);
  satellite
}

fn main() {
  let base: GroundStation = GroundStation {};
  let mut sat_a: CubeSat = CubeSat {
    id: 0,
    mailbox: Mailbox { messages: vec![] },
  };

  println!("t0: {:?}", sat_a);

  base.send(&mut sat_a, Message::from("hello there!"));
  println!("t1: {:?}", sat_a);

  let msg: Option<_> = sat_a.recv();
  println!("t2: {:?}", sat_a);

  println!("msg {:?}", msg);
}
