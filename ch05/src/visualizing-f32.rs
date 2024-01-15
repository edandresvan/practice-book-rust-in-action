fn _before() {
  let n: f32 = 42.42;
  let n_bits: u32 = n.to_bits();
  // Isolate the sign
  let sign_bit: u32 = n_bits >> 31;

  // Isolate the exponent
  let exponent: u32 = n_bits >> 23;
  let exponent: u32 = exponent & 0xff;
  let exponent: i32 = (exponent as i32) - 127;

  // Isolate the mantisa
  let mut mantissa: f32 = 1.0;

  for i in 0..23 {
    let mask = 1 << i;
    let one_at_bit_i = n_bits & mask;
    if one_at_bit_i != 0 {
      let i2: f32 = i as f32;
      let weight = 2_f32.powf(i2 - 23.0);
      mantissa += weight;
    }
  }
  println!("{}", exponent);
}

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn to_parts(number: f32) -> (u32, u32, u32) {
  let bits: u32 = number.to_bits();

  // The sign is the first bit, so move away all those bits all the remaning bits and preserve the sign bit
  let sign: u32 = (bits >> 31) & 1;

  // The exponent is in the following eight bits after the sign. So move away all the remaing bits and eliminate the sign bit
  let exponent: u32 = (bits >> 23) & 0xff;

  // The fractional part is located in the last 23 bits so use an AND mask to discard the first 9 bits
  let fraction: u32 = bits & 0x7fffff;

  (sign, exponent, fraction)
}

fn decode(
  sign: u32,
  exponent: u32,
  fraction: u32,
) -> (f32, f32, f32) {
  // The sign depends on raising -1 to a power:
  // (-1) ^ 0 =  1: zero or positive number
  // (-1) ^ 1 = -1: negative number.
  let signed_1: f32 = (-1.0_f32).powf(sign as f32);

  let exponent: i32 = (exponent as i32) - BIAS;
  let exponent: f32 = RADIX.powf(exponent as f32);
  let mut mantissa: f32 = 1.0;

  for i in 0..23 {
    let mask: u32 = 1 << i;
    let one_at_bit_i = fraction & mask;
    if one_at_bit_i != 0 {
      let i_: f32 = i as f32;
      let weight: f32 = 2_f32.powf(i_ - 23.0);
      mantissa += weight;
    }
  }

  (signed_1, exponent, mantissa)
}

fn from_parts(
  sign: f32,
  exponent: f32,
  mantissa: f32,
) -> f32 {
  sign * exponent * mantissa
}

fn main() {
  let n: f32 = -172.42;
  let (sign_part, exponent_part, fraction_part) = to_parts(n);
  let (sign_decoded, exponent_decoded, mantissa_decoded) =
    decode(sign_part, exponent_part, fraction_part);
  let n_decoded: f32 = from_parts(sign_decoded, exponent_decoded, mantissa_decoded);

  println!("original number: {n} | decoded number: {n_decoded}");
  println!();

  println!("⍊ component ⍊{: <19} as bits ⍊ as real number ⍊", "");
  println!(
    "|sign       |{: <24}  {:01b} | {: <11} {} |",
    "", sign_part, "", sign_decoded
  );
  println!(
    "|exponent   |{: <17}  {:<08b} | {: <10} {} |",
    "", exponent_part, "", exponent_decoded
  );
  println!(
    "|mantissa   |{: <1}   {:023b} | {: <4} {} |",
    "", fraction_part, "", mantissa_decoded
  );
}
