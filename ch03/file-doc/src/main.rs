//! Simulating a file object

#[derive(Debug)]
/// Represents a file on disk, memory, or network.
pub struct File {
  name: String,
  data: Vec<u8>,
}

impl File {
  /// Creates a new empty file with the given name.
  ///
  /// # Example
  ///
  /// ```
  /// let f: File = File::new("f.txt");
  /// ```
  pub fn new(name: &str) -> Self {
    Self {
      name: String::from(name),
      data: Vec::new(),
    }
  }

  /// Gets the file's length in bytes.
  pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Gets the file's name.
  pub fn name(&self) -> String {
    self.name.clone()
  }
}

fn main() {
  let f1: File = File::new("f1.txt");

  println!("{:?}", &f1);
  println!("{} is {} bytes long", f1.name(), f1.len());
}
