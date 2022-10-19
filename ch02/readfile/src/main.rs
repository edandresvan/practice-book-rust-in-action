use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
fn read_file() {
  let my_file: File = File::open("./readme.md").unwrap();
  let mut reader: BufReader<File> = BufReader::new(my_file);
  let mut line = String::new();

  loop {
    let len: usize = reader.read_line(&mut line).unwrap();

    if len == 0 {
      break;
    }

    println!("{} ({} bytes long)", line, len);
    line.truncate(0);
  }
}

#[allow(dead_code)]
fn bufreader_lines() {
  let my_file: File = File::open("./readme.md").unwrap();
  let reader: BufReader<File> = BufReader::new(my_file);

  for line in reader.lines() {
    let text_line: String = line.unwrap();
    println!("{} ({} bytes long)", text_line, text_line.len());
  }
}

fn main() {
  // read_file();
  bufreader_lines();
}
