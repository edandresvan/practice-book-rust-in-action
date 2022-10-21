#[derive(Debug)]
enum Event {
  Update,
  Delete,
  Unknown,
}

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
  let parts: Vec<&str> = line.splitn(2, ' ').collect();

  if parts.len() == 1 {
    return (Event::Unknown, String::from(line));
  }

  let event: &str = parts[0];
  let rest: String = String::from(parts[1]);

  match event.to_lowercase().as_str() {
   "update" => (Event::Update, rest),
   "delete" => (Event::Delete, rest),
    _ => (Event::Unknown, String::from(line)),
  }
}

fn main() {
  let log: &str = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111
update 101 tshirt 3
delete 201 tie
upDATe 102 pants 2
deLEtE 202 collar";

  for line in log.lines() {
    let parse_result: (Event, Message) = parse_log(line);
    println!("{:?}", parse_result);
  }
}
