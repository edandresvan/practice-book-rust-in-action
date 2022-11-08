#![allow(unused_variables)]

use std::vec;

#[derive(Debug)]
struct CubeSat {
  id: u64,
}

#[derive(Debug)]
struct Message {
  to: u64,
  content: String,
}

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

struct GroundStation {}

impl Mailbox {
  fn post(
    &mut self,
    msg: Message,
  ) {
    self.messages.push(msg);
  }

  fn deliver(
    &mut self,
    recipient: &CubeSat,
  ) -> Option<Message> {
    for i in 0..self.messages.len() {
      if self.messages[i].to == recipient.id {
        let msg: Message = self.messages.remove(i);
        return Some(msg);
      }
    }

    None
  }
}

impl GroundStation {
  fn connect(
    &self,
    satellite_id: u64,
  ) -> CubeSat {
    CubeSat { id: satellite_id }
  }

  fn send(
    &self,
    mailbox: &mut Mailbox,
    msg: Message,
  ) {
    mailbox.post(msg);
  }
}

impl CubeSat {
  fn recv(
    &self,
    mailbox: &mut Mailbox,
  ) -> Option<Message> {
    mailbox.deliver(&self)
  }
}

fn fetch_sat_ids() -> Vec<u64> {
  vec![1, 2, 3]
}

fn main() {
  let mut mail: Mailbox = Mailbox { messages: vec![] };

  let base: GroundStation = GroundStation {};

  let sat_ids: Vec<u64> = fetch_sat_ids();

  for sat_id in sat_ids {
    let satellite: CubeSat = base.connect(sat_id);

    let msg: Message = Message {
      to: satellite.id,
      content: String::from("hello"),
    };

    base.send(&mut mail, msg);
  }

  let sat_ids: Vec<u64> = fetch_sat_ids();

  for sat_id in sat_ids {
    let satellite: CubeSat = base.connect(sat_id);

    let msg: Option<Message> = satellite.recv(&mut mail);

    println!("{:?}: {:?}", satellite, msg);
  }
}
