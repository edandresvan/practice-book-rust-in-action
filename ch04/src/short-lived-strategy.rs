#![allow(unused_variables)]

#[derive(Debug)]
struct Message {
  to: u64,
  content: String,
}

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

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
        let msg = self.messages.remove(i);
        return Some(msg);
      }
    }
    None
  }
}

#[derive(Debug)]
struct CubeSat {
  id: u64,
}

struct GroundStation {}

impl GroundStation {
  fn send(
    &self,
    mailbox: &mut Mailbox,
    msg: Message,
  ) {
    mailbox.post(msg);
  }

  fn connect(
    &self,
    satellite_id: u64,
  ) -> CubeSat {
    CubeSat { id: satellite_id }
  }
}

impl CubeSat {
  fn recv(
    &self,
    mailbox: &mut Mailbox,
  ) -> Option<Message> {
    mailbox.deliver(self)
  }
}

fn fetch_satellite_ids() -> Vec<u64> {
  vec![1, 2, 3]
}

impl GroundStation {}

fn main() {
  let base = GroundStation {};
  let mut mail = Mailbox { messages: vec![] };

  let satellite_ids = fetch_satellite_ids();

  for sat_id in satellite_ids {
    let satellite: CubeSat = base.connect(sat_id);
    let msg = Message {
      to: sat_id,
      content: "hello".to_string(),
    };
    base.send(&mut mail, msg);
  }

  let satellite_ids = fetch_satellite_ids();

  for sat_id in satellite_ids {
    let satellite = base.connect(sat_id);
    let msg = satellite.recv(&mut mail);
    println!("{:?}, {:?}", satellite, msg);
  }
}
