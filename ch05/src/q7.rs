#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Q7(i8);

impl From<f64> for Q7 {
  fn from(value: f64) -> Self {
    if value >= 1.0 {
      Q7(127)
    } else if value <= -1.0 {
      Q7(-128)
    } else {
      Q7((value * 128.0) as i8)
    }
  }
}

impl From<Q7> for f64 {
  fn from(value: Q7) -> Self {
    (value.0 as f64) * 2_f64.powf(-7.0)
  }
}

impl From<f32> for Q7 {
  fn from(value: f32) -> Self {
    Q7::from(value as f64)
  }
}

impl From<Q7> for f32 {
  fn from(value: Q7) -> Self {
    f64::from(value) as f32
  }
}

fn main() {
  println!("Please, execute this command: cargo test --bin q7");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn out_of_bounds() {
    assert_eq!(Q7::from(10.0), Q7::from(1.0));
    assert_eq!(Q7::from(10.0), Q7(127));
    assert_eq!(Q7::from(1.0), Q7(127));

    assert_eq!(Q7::from(-10.0), Q7::from(-1.0));
    assert_eq!(Q7::from(-10.0), Q7(-128));
    assert_eq!(Q7::from(-1.0), Q7(-128));
  }

  #[test]
  fn f32_to_q7() {
    let n1: f32 = 0.7;
    let q1: Q7 = Q7::from(n1);

    let n2: f32 = -0.4;
    let q2: Q7 = Q7::from(n2);

    let n3: f32 = 123.0;
    let q3: Q7 = Q7::from(n3);

    assert_eq!(q1, Q7(89));
    assert_eq!(q2, Q7(-51));
    assert_eq!(q3, Q7(127));
  }

  #[test]
  fn q7_to_f32() {
    let q1: Q7 = Q7::from(0.85);
    let n1: f32 = f32::from(q1);
    assert_eq!(n1, 0.84375);

    let q2: Q7 = Q7::from(n1);
    let n2: f32 = f32::from(q2);
    assert_eq!(n1, n2);
    assert_eq!(q2, Q7(108));
  }
}
