// You are given an integer array nums and an integer k.

// Return an integer denoting the sum of all elements in nums whose frequency is divisible by k, or 0 if there are no such elements.

// Note: An element is included in the sum exactly as many times as it appears in the array if its total frequency is divisible by k.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100
// 1 <= k <= 100

struct Solution;
impl Solution {
    pub fn sum_divisible_by_k(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let n = nums.len();
        let mut ans = 0;
        let mut cnt = 1;

        for i in 1..=n {
            if i == n || nums[i] != nums[i - 1] {
                if cnt % k == 0 {
                    ans += nums[i - 1] * cnt;
                }
                cnt = 0;
            }
            cnt += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 2, 3, 3, 3, 3, 4];
        let k = 2;
        let result = Solution::sum_divisible_by_k(nums, k);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;
        let result = Solution::sum_divisible_by_k(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![4, 4, 4, 1, 2, 3];
        let k = 3;
        let result = Solution::sum_divisible_by_k(nums, k);
        assert_eq!(result, 12);
    }
}
