// Given two positive integers a and b, return the number of common factors of a and b.

// An integer x is a common factor of a and b if x divides both a and b.

struct Solution;
impl Solution {
  pub fn common_factors(a: i32, b: i32) -> i32 {
    let gcd = if a > b {
      Self::find_gcd(a, b)
    } else {
      Self::find_gcd(b, a)
    };

    let mut ans = 1;
    for num in 1..gcd {
      if gcd % num == 0 {
        ans += 1;
      }
    }
    ans
  }

  fn find_gcd(a: i32, b: i32) -> i32 {
      return if b == 0 {
        a
      } else {
        Self::find_gcd(b, a % b)
      }
    }
}