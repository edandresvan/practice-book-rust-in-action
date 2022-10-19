fn main() {
  let mut letters: Vec<&str> = vec!["a", "b", "c"];

  for letter in letters {
    println!("{}", letter);
    letters.push(letter.clone());
  }
}
