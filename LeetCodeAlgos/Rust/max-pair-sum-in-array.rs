// You are given an integer array nums. You have to find the maximum sum of a pair of numbers from nums such that the largest digit in both numbers is equal.

// For example, 2373 is made up of three distinct digits: 2, 3, and 7, where 7 is the largest among them.

// Return the maximum sum or -1 if no such pair exists.

struct Solution;

use std::collections::{
  hash_map::Entry,
  HashMap
};
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
      let mut map = HashMap::<i32, i32>::new();

      let mut ans = -1;
      for num in nums {
        let max_digit = {
          let mut n = num;
          let mut m = 0;
          while n > 0 {
            m = m.max(n % 10);
            n /= 10;
          }
          m
        };

        match map.entry(max_digit) {
          Entry::Occupied(mut occ) => {
            let val = occ.get_mut();
            ans = ans.max(*val + num);
            if num > *val {
              *val = num;
            }
          },
          Entry::Vacant(vac) => {
            vac.insert(num);
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
    fn test1() {
        let nums = vec![112,131,411];
        assert_eq!(Solution::max_sum(nums), -1);
    }

    #[test]
    fn test2() {
        let nums = vec![2536,1613,3366,162];
        assert_eq!(Solution::max_sum(nums), 5902);
    }

    #[test]
    fn test3() {
        let nums = vec![51,71,17,24,42];
        assert_eq!(Solution::max_sum(nums), 88);
    }
}
