const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn to_parts(n: f32) -> (u32, u32, u32) {
  let bits: u32 = n.to_bits();

  let sign: u32 = (bits >> 31) & 1;
  let exponent: u32 = (bits >> 23) & 0b11111111; // 0xff
  let fraction: u32 = bits & 0x7fffff;

  (sign, exponent, fraction)
}

fn from_parts(
  sign: f32,
  exponent: f32,
  mantissa: f32,
) -> f32 {
  sign * exponent * mantissa
}

fn decode(
  sign: u32,
  exponent: u32,
  fraction: u32,
) -> (f32, f32, f32) {
  let signed_1: f32 = (-1.0_f32).powf(sign as f32);

  let exponent: i32 = (exponent as i32) - BIAS;
  let exponent: f32 = RADIX.powf(exponent as f32);

  let mut mantissa: f32 = 1.0;
  for i in 0..23 {
    let mask: u32 = 1 << i;
    let one_at_bit_i: u32 = fraction & mask;

    if one_at_bit_i != 0 {
      let i_: f32 = i as f32;
      let weigth: f32 = 2_f32.powf(i_ - 23.0);
      mantissa += weigth;
    }
  }

  (signed_1, exponent, mantissa)
}

fn main() {
  let n: f32 = 42.42;

  let (sign, exp, frac) = to_parts(n);
  let (sign_, exp_, mant_) = decode(sign, exp, frac);

  let n_ = from_parts(sign_, exp_, mant_);

  println!("{} -> {}", n, n_);

  println!("field     |{: <19} as bits | as real number", "");
  println!("sign      |{: <24}  {:01b} | {}", "", sign, sign_);
  println!("exponent  |{: <18}  {:<08b}| {}", "", exp, exp_);
  println!("mantisa   |{: <2}   {:023b}| {}", "", frac, mant_);
}
