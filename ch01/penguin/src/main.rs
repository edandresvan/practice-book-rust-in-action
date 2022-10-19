fn main() {
  let penguin_data: &str = "\
  common name, length (cm)
  Little penguin,33
  Yellow-eye penguin,65
  Fiorland penguin,60
  Invalid,data
  ";

  let records = penguin_data.lines();

  for (i, record) in records.enumerate() {
    if i == 0 || record.trim().len() == 0 {
      continue;
    }

    let fields: Vec<&str> = record.split(",").map(|field| field.trim()).collect();

    if cfg!(debug_assertions) {
      eprintln!("debug: {:?} -> {:?}", record, fields);
    }

    let name = fields[0];
    if let Ok(length) = fields[1].parse::<f32>() {
      println!("{}, {}cm", name, length);
    }
  }
}
