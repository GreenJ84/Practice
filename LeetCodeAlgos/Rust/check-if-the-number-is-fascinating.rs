// You are given an integer n that consists of exactly 3 digits.

// We call the number n fascinating if, after the following modification, the resulting number contains all the digits from 1 to 9 exactly once and does not contain any 0's:

// Concatenate n with the numbers 2 * n and 3 * n.
// Return true if n is fascinating, or false otherwise.

// Concatenating two numbers means joining them together. For example, the concatenation of 121 and 371 is 121371.

struct Solution;
impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut count = [0; 9];
        for ch in (
          n.to_string() + &(2 * n).to_string() + &(3 * n).to_string()
        ).chars() {
          let digit = (ch as u8 - b'0') as usize;
          if digit == 0 {
              return false;
          }
          count[digit - 1] += 1;
        }
        count.iter().all(|x| x == &1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_fascinating(192), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_fascinating(100), false);
    }
}