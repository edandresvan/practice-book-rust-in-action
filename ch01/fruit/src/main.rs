fn main() {
  let fruits: Vec<&str> = vec!["kiwi", "banana", "grape"];

  let buffer_overflow: &str = fruits[4];
  assert_eq!(buffer_overflow, "watermellon");
}
