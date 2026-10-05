// You are given a 1-indexed integer array nums of length n.

// An element nums[i] of nums is called special if i divides n, i.e. n % i == 0.

// Return the sum of the squares of all special elements of nums.

// Constraints:
// 1 <= nums.length == n <= 50
// 1 <= nums[i] <= 50

struct Solution;
impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let n = nums.len();
        for (i, &num) in nums.iter().enumerate() {
            if n % (i + 1) == 0 {
                println!("{num}");
                sum += num * num;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::sum_of_squares(nums);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 7, 1, 19, 18, 3];
        let result = Solution::sum_of_squares(nums);
        assert_eq!(result, 63);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let result = Solution::sum_of_squares(nums);
        assert_eq!(result, 50);
    }
}
