// You are given an integer array nums. Transform nums by performing the following operations in the exact order specified:

// Replace each even number with 0.
// Replace each odd numbers with 1.
// Sort the modified array in non-decreasing order.
// Return the resulting array after performing these operations.

struct Solution;
impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut idx = 0usize;
        for num in &nums {
          if num % 2 == 0 {
            idx += 1;
          }
        }
        println!("{}", idx);
        for i in idx..nums.len() {
          ans[i] = 1;
        }
        ans
    }

    pub fn _transform_array1(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::<i32>::new();
        let mut ones = 0usize;
        for num in nums {
          if num % 2 == 0 {
            ans.push(0);
          } else {
            ones += 1;
          }
        }
        for _ in 0..ones {
          ans.push(1);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4,3,2,1];
        assert_eq!(Solution::transform_array(nums), vec![0,0,1,1]);
    }

    #[test]
    fn test2() {
        let nums = vec![1,5,1,4,2];
        assert_eq!(Solution::transform_array(nums), vec![0,0,1,1,1]);
    }
}
