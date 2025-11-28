// A web developer needs to know how to design a web page's size. So, given a specific rectangular web page’s area, your job by now is to design a rectangular web page, whose length L and width W satisfy the following requirements:

// - The area of the rectangular web page you designed must equal to the given target area.
// - The width W should not be larger than the length L, which means L >= W.
// - The difference between length L and width W should be as small as possible.

// Return an array [L, W] where L and W are the length and width of the web page you designed in sequence.

struct Solution;
impl Solution {
  pub fn construct_rectangle(area: i32) -> Vec<i32> {
    let start = (area as f64).sqrt() as i32;

    for width in (0..=start).rev() {
      if area % width == 0 {
        return vec![area / width, width];
      }
    }
    vec![-1, -1]
  }
}


#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn area_1() {
    assert_eq!(Solution::construct_rectangle(1), vec![1, 1]);
  }

  #[test]
  fn area_2() {
    assert_eq!(Solution::construct_rectangle(2), vec![2, 1]);
  }

  #[test]
  fn area_3() {
    assert_eq!(Solution::construct_rectangle(3), vec![3, 1]);
  }

  #[test]
  fn area_4() {
    assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
  }

  #[test]
  fn area_8() {
    assert_eq!(Solution::construct_rectangle(8), vec![4, 2]);
  }

  #[test]
  fn area_13_prime() {
    assert_eq!(Solution::construct_rectangle(13), vec![13, 1]);
  }

  #[test]
  fn area_16_perfect_square() {
    assert_eq!(Solution::construct_rectangle(16), vec![4, 4]);
  }

  #[test]
  fn area_36_perfect_square() {
    assert_eq!(Solution::construct_rectangle(36), vec![6, 6]);
  }

  #[test]
  fn area_37_from_example() {
    assert_eq!(Solution::construct_rectangle(37), vec![37, 1]);
  }

  #[test]
  fn area_122122_from_example() {
    assert_eq!(Solution::construct_rectangle(122122), vec![427, 286]);
  }

  #[test]
  fn area_1_000_000_large_square() {
    assert_eq!(Solution::construct_rectangle(1_000_000), vec![1000, 1000]);
  }
}