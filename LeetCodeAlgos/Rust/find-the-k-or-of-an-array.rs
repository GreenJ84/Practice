// You are given an integer array nums, and an integer k. Let's introduce K-or operation by extending the standard bitwise OR. In K-or, a bit position in the result is set to 1 if at least k numbers in nums have a 1 in that position.

// Return the K-or of nums.

// Constraints:
// 1 <= nums.length <= 50
// 0 <= nums[i] < 231
// 1 <= k <= nums.length

struct Solution;
impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = [0i32; 32];
        for mut num in nums {
            let mut i = ans.len() - 1;
            let mut maxed = 0;
            while num > 0 {
                if ans[i] < k {
                    ans[i] += num & 1;
                } else {
                    maxed += 1;
                }
                num >>= 1;
                i -= 1;
            }
            if maxed == ans.len() {
                return i32::MAX
            }
        }
        ans.iter()
            .map(|&bit| if bit >= k { true } else { false })
            .fold(0i32, |acc, bit| (acc << 1) | (bit as i32))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![7, 12, 9, 8, 9, 15];
        let k: i32 = 4;
        let expected = 9;
        assert_eq!(Solution::find_k_or(nums, k), expected);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![2, 12, 1, 11, 4, 5];
        let k: i32 = 6;
        let expected = 0;
        assert_eq!(Solution::find_k_or(nums, k), expected);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![10, 8, 5, 9, 11, 6, 8];
        let k: i32 = 1;
        let expected = 15;
        assert_eq!(Solution::find_k_or(nums, k), expected);
    }
}
