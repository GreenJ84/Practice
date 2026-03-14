// Given an integer n, return any array containing n unique integers such that they add up to 0.

struct Solution;
impl Solution {
    pub fn sum_zero(mut n: i32) -> Vec<i32> {
      let mut ans = Vec::new();
      if n % 2 == 1 {
        ans.push(0);
        n -= 1;
      }
      for num in 1..=(n/2){
        ans.push(num);
        ans.push(-num);
      }
      ans
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let sol = Solution::sum_zero(5);
    assert_eq!(sol.len(), 5);
    assert_eq!(sol.iter().sum::<i32>(), 0);
  }

    #[test]
  fn example_2() {
    let sol = Solution::sum_zero(3);
    assert_eq!(sol.len(), 3);
    assert_eq!(sol.iter().sum::<i32>(), 0);
  }

    #[test]
  fn example_3() {
    let sol = Solution::sum_zero(1);
    assert_eq!(sol.len(), 1);
    assert_eq!(sol.iter().sum::<i32>(), 0);
  }
}

// Example 1:

// Input: n = 5
// Output: [-7,-1,1,3,4]
// Explanation: These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
// Example 2:

// Input: n = 3
// Output: [-1,0,1]
// Example 3:

// Input: n = 1
// Output: [0]
