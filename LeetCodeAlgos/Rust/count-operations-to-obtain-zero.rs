// You are given two non-negative integers num1 and num2.

// In one operation, if num1 >= num2, you must subtract num2 from num1, otherwise subtract num1 from num2.

// For example, if num1 = 5 and num2 = 4, subtract num2 from num1, thus obtaining num1 = 1 and num2 = 4. However, if num1 = 4 and num2 = 5, after one operation, num1 = 4 and num2 = 1.
// Return the number of operations required to make either num1 = 0 or num2 = 0.

// Constraints:
// 0 <= num1, num2 <= 105

struct Solution;
impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut ans = 0;
        while num1 > 0 && num2 > 0 {
          if num1 == num2 {
            return ans + 1;
          } else if num1 == 1 || num2 == 1 {
            return ans + num1.max(num2);
          } else {
            ans += 1;
            if num1 > num2 {
              num1 -= num2;
            } else {
              num2 -= num1;
            }
          }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let num1: i32 = 2;
    let num2: i32 = 3;
    assert_eq!(Solution::count_operations(num1, num2), 3);
  }

  #[test]
  fn test_2() {
    let num1: i32 = 10;
    let num2: i32 = 10;
    assert_eq!(Solution::count_operations(num1, num2), 1);
  }
}
