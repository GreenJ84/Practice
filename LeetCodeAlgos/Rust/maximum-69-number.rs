// You are given a positive integer num consisting only of digits 6 and 9.

// Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).

struct Solution;
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut one = false;
        num.to_string()
          .chars()
          .map(|ch| {
              match (ch, one) {
                  ('6', false) => {
                      one = true;
                      '9'
                  }
                  (x, _) => { x }
              }
          })
          .collect::<String>()
          .parse::<i32>()
          .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::maximum69_number(9996), 9999);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::maximum69_number(9999), 9999);
    }
}
