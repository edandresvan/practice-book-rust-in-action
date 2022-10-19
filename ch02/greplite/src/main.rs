use std::fs::File;
use std::io::{prelude::*, stdin, BufReader};

use clap::{arg, Command};
use regex::{Match, Regex};

#[allow(dead_code)]
fn simple_pattern() {
  let search_term: &str = "picture";
  let quote: &str = "\
  Every face, every shop, bedroom window, public-house, and
  dark square is a picture feverishly turned--in search of what?
  It is the same with books.
  What do we seek through millions of pages?";

  for line in quote.lines() {
    if line.contains(search_term) {
      println!("{}", line);
    }
  }
}

#[allow(dead_code)]
fn simple_linenums() {
  let search_term: &str = "picture";
  let quote: &str = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

  for (i, line) in quote.lines().enumerate() {
    if line.contains(search_term) {
      let line_num: usize = i + 1;
      println!("{}: {}", line_num, line);
    }
  }
}

#[allow(dead_code)]
fn context_vec() {
  let ctx_lines: usize = 2;
  let needle: &str = "oo";
  let haystack: &str = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

  let mut tags: Vec<usize> = vec![];

  let mut ctx: Vec<Vec<(usize, String)>> = vec![];

  for (line_number, line) in haystack.lines().enumerate() {
    if line.contains(needle) {
      tags.push(line_number);

      let v: Vec<(usize, String)> = Vec::with_capacity(2 * ctx_lines + 1);
      ctx.push(v);
    }
  }

  if tags.is_empty() {
    return;
  }

  for (line_number, line) in haystack.lines().enumerate() {
    for (tag_number, tag) in tags.iter().enumerate() {
      let lower_bound: usize = tag.saturating_sub(ctx_lines);
      let upper_bound: usize = tag + ctx_lines;

      if (line_number >= lower_bound) && (line_number <= upper_bound) {
        let line_as_string: String = String::from(line);
        let local_ctx: (usize, String) = (line_number, line_as_string);
        ctx[tag_number].push(local_ctx);
      }
    }
  }

  for local_ctx in ctx.iter() {
    for &(i, ref line) in local_ctx.iter() {
      let line_number: usize = i + 1;
      println!("{}: {}", line_number, line);
    }
  }
}

#[allow(dead_code)]
fn match_regex() {
  let re: Regex = Regex::new("square").unwrap();

  let quote: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

  for line in quote.lines() {
    let contains_substring: Option<Match> = re.find(line);
    match contains_substring {
      Some(_) => println!("{}", line),
      None => (),
    }
  }
}
#[allow(dead_code)]
fn clap_regex() {
  let matches: clap::ArgMatches = Command::new("grep-lite")
    .about("Searches for patterns")
    .arg(arg!([pattern] "Pattern to search for").required(true))
    .get_matches();

  if let Some(pattern) = matches.get_one::<String>("pattern") {
    let re: Regex = Regex::new(pattern).unwrap();

    let quote: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
      match re.find(line) {
        None => (),
        Some(_) => {
          println!("{}", line);
        }
      }
    }
  }
}

#[allow(dead_code)]
fn clap_file() {
  let matches: clap::ArgMatches = Command::new("grep-lite")
    .about("Searches for patterns")
    .arg(arg!([pattern] "Pattern to search for").required(true))
    .arg(arg!([input] "File to search").required(true))
    .get_matches();

  let pattern: &String = matches.get_one("pattern").unwrap();

  let input: &String = matches.get_one("input").unwrap();

  let re: Regex = Regex::new(pattern).unwrap();
  let my_file: File = File::open(input).unwrap();

  let reader: BufReader<File> = BufReader::new(my_file);

  for buffer_line in reader.lines() {
    let line_text: String = buffer_line.unwrap();

    // match re.find(&line_text) {
    //   Some(_) => {
    //     println!("{}", line_text);
    //   }
    //   None => (),
    // }

    if let Some(_) = re.find(&line_text) {
      println!("{}", line_text);
    }
  }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
  for buffer_line in reader.lines() {
    let line_text: String = buffer_line.unwrap();

    if let Some(_) = re.find(&line_text) {
      println!("{}", line_text);
    }
  }
}

#[allow(dead_code)]
fn clap_file_stdin() {
  let matches: clap::ArgMatches = Command::new("grep-lite")
    .about("Searches for patterns")
    .arg(arg!([pattern] "Pattern to search for").required(true))
    .arg(arg!([input] "File to search").required(false))
    .get_matches();

  let pattern: &String = matches.get_one::<String>("pattern").unwrap();

  let default_filename: String = String::from("-");
  let input: &String = matches.get_one::<String>("input").unwrap_or(&default_filename);

  let re: Regex = Regex::new(pattern).unwrap();

  if input == "-" {
    let stdin_input = stdin();
    let reader = stdin_input.lock();
    process_lines(reader, re);
  } else {
    let my_file: File = File::open(input).unwrap();
    let reader: BufReader<File> = BufReader::new(my_file);
    process_lines(reader, re);
  }
}

fn main() {
  // simple_pattern();
  // simple_linenums();
  // context_vec();
  // match_regex();
  // clap_regex();
  // clap_file();
  clap_file_stdin();
}
