// You are given an integer n.

// An integer is called Monobit if all bits in its binary representation are the same.

// Return the count of Monobit integers in the range [0, n] (inclusive).

// Constraints:
// 0 <= n <= 1000

struct Solution;
impl Solution {
    pub fn count_monobit(n: i32) -> i32 {
        match n {
          0 => 1,
          x if x < 3 => 2,
          x if x < 7 => 3,
          x if x < 15 => 4,
          x if x < 31 => 5,
          x if x < 63 => 6,
          x if x < 127 => 7,
          x if x < 255 => 8,
          x if x < 511 => 9,
          _ => 10
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let n = 1;
    let result = Solution::count_monobit(n);
    assert_eq!(result, 2);
  }

  #[test]
  fn test_2() {
    let n = 4;
    let result = Solution::count_monobit(n);
    assert_eq!(result, 3);
  }
}
