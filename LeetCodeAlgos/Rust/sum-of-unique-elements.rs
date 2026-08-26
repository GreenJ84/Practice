// You are given an integer array nums. The unique elements of an array are the elements that appear exactly once in the array.

// Return the sum of all the unique elements of nums.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100

struct Solution;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut check = vec![0; 100];
        for &num in &nums {
            let idx = (num as usize) - 1;
            check[idx] += 1;
            match check[idx] {
                1 => {
                    ans += num;
                }
                2 => {
                    ans -= num;
                }
                _ => {}
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
        let nums = vec![1, 2, 3, 2];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 15);
    }
}
