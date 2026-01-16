struct Solution;
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let (col, row) = {
            let mut chars = coordinates.chars();
            (
              chars.next().unwrap() as u8 - b'a' + 1,
              chars.next().unwrap() as u8 - b'0',
            )
        };
        (col + row) % 2 != 0
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_a1() {
    assert_eq!(Solution::square_is_white("a1".to_string()), false);
  }

  #[test]
  fn test_h3() {
    assert_eq!(Solution::square_is_white("h3".to_string()), true);
  }

  #[test]
  fn test_c7() {
    assert_eq!(Solution::square_is_white("c7".to_string()), false);
  }

  #[test]
  fn test_a8() {
    assert_eq!(Solution::square_is_white("a8".to_string()), true);
  }

  #[test]
  fn test_h8() {
    assert_eq!(Solution::square_is_white("h8".to_string()), false);
  }

  #[test]
  fn test_d4() {
    assert_eq!(Solution::square_is_white("d4".to_string()), false);
  }

  #[test]
  fn test_e5() {
    assert_eq!(Solution::square_is_white("e5".to_string()), false);
  }

  #[test]
  fn test_f6() {
    assert_eq!(Solution::square_is_white("f6".to_string()), false);
  }
}