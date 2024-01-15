fn mock_rand(number: u8) -> f32 {
  let base: u32 = 0b0011_1111_0000_0000_0000_0000_0000_0000;
  let large_number: u32 = (number as u32) << 15;
  let f32_bits = base | large_number;
  let m: f32 = f32::from_bits(f32_bits);
  2.0 * (m - 0.5)
}
fn main() {
  println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
  println!("max of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
  println!("max of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}
