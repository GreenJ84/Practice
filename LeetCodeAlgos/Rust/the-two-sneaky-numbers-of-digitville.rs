// In the town of Digitville, there was a list of numbers called nums containing integers from 0 to n - 1. Each number was supposed to appear exactly once in the list, however, two mischievous numbers sneaked in an additional time, making the list longer than usual.

// As the town detective, your task is to find these two sneaky numbers. Return an array of size two containing the two numbers (in any order), so peace can return to Digitville.

struct Solution;
impl Solution {
    pub fn get_sneaky_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut ans = Vec::new();
        for i in 1..nums.len() {
          if nums[i] == nums[i-1] {
            ans.push(nums[i]);
          }
        }
        ans
    }

      pub fn _get_sneaky_numbers1(nums: Vec<i32>) -> Vec<i32> {
        let mut check = vec![false; nums.len() - 2];
        let mut ans = Vec::new();
        for num in nums {
          if check[num as usize] {
            ans.push(num);
          }
          check[num as usize] = true;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![0, 1, 1, 0];
        let result = Solution::get_sneaky_numbers(nums);
        assert!(result.eq(&vec![0, 1]) || result.eq(&vec![1, 0]));
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 3, 2, 1, 3, 2];
        let result = Solution::get_sneaky_numbers(nums);
        assert!(result.eq(&vec![2, 3]) || result.eq(&vec![3, 2]));
    }

    #[test]
    fn test_example_3() {
        let nums = vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2];
        let result = Solution::get_sneaky_numbers(nums);
        assert!(result.eq(&vec![4, 5]) || result.eq(&vec![5, 4]));
    }
}
