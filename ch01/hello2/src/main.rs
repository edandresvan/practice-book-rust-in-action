fn greet_world() {
  println!("Hello, world!");
  let southern_germany: &str  = "Grüß Gott!";
  let japan: &str = "ハロー・ワールド";
  let regions: [&str; 2] = [southern_germany, japan];

  for region in regions.iter() {
    println!("{}", &region);
  }
}

fn main() {
  greet_world();
}
