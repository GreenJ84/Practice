// You are given an integer array prices representing the daily price history of a stock, where prices[i] is the stock price on the ith day.

// A smooth descent period of a stock consists of one or more contiguous days such that the price on each day is lower than the price on the preceding day by exactly 1. The first day of the period is exempted from this rule.

// Return the number of smooth descent periods.


struct Solution;
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut ans = prices.len() as i64;

        let mut run = 1usize;
        for idx in 1..prices.len(){
          if prices[idx] == prices[idx - 1] - 1{
            run += 1;
            continue;
          }
          ans += ((run - 1) * run / 2) as i64;
          run = 1;
        }

        ans + ((run - 1) * run / 2) as i64
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(Solution::get_descent_periods(vec![3, 2, 1, 4]), 7);
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::get_descent_periods(vec![8, 6, 7, 7]), 4);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::get_descent_periods(vec![1]), 1);
  }

  #[test]
  fn single_element() {
    assert_eq!(Solution::get_descent_periods(vec![100]), 1);
  }

  #[test]
  fn two_elements_descent() {
    assert_eq!(Solution::get_descent_periods(vec![5, 4]), 3);
  }

  #[test]
  fn two_elements_no_descent() {
    assert_eq!(Solution::get_descent_periods(vec![5, 3]), 2);
  }

  #[test]
  fn long_descent_sequence() {
    assert_eq!(Solution::get_descent_periods(vec![5, 4, 3, 2, 1]), 15);
  }

  #[test]
  fn all_same_prices() {
    assert_eq!(Solution::get_descent_periods(vec![5, 5, 5, 5]), 4);
  }

  #[test]
  fn ascending_prices() {
    assert_eq!(Solution::get_descent_periods(vec![1, 2, 3, 4]), 4);
  }

  #[test]
  fn alternating_pattern() {
    assert_eq!(Solution::get_descent_periods(vec![5, 4, 5, 4]), 6);
  }
}
