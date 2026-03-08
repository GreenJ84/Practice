// You are given integers n, m, and k.

// There are two logs of lengths n and m units, which need to be transported in three trucks where each truck can carry one log with length at most k units.

// You may cut the logs into smaller pieces, where the cost of cutting a log of length x into logs of length len1 and len2 is cost = len1 * len2 such that len1 + len2 = x.

// Return the minimum total cost to distribute the logs onto the trucks. If the logs don't need to be cut, the total cost is 0.

struct Solution;
impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        let mut cost = 0;
        if n > k {
            let diff = n - k;
            cost += k as i64 * diff as i64;
        }
        if m > k {
            let diff = m - k;
            cost += k as i64 * diff as i64;
        }
        cost
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let (n, m, k) = (6, 5, 5);
    let expected = 5;
    assert_eq!(Solution::min_cutting_cost(n, m, k), expected);
  }

  #[test]
  fn test_example_2() {
    let (n, m, k) = (4, 4, 6);
    let expected = 0;
    assert_eq!(Solution::min_cutting_cost(n, m, k), expected);
  }
}