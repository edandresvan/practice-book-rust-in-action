fn main() {
  let needle: i32 = 0o204; // 132
  let haystack: [i32; 10] = [1, 1, 2, 5, 15, 52, 132, 877, 4140, 21147];

  for item in &haystack {
    if *item == needle {
      println!("{}", item);
    }
  }
}
