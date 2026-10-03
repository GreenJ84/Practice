// You are given a 0-indexed integer array nums. In one operation, you may do the following:

// Choose two integers in nums that are equal.
// Remove both integers from nums, forming a pair.
// The operation is done on nums as many times as possible.

// Return a 0-indexed integer array answer of size 2 where answer[0] is the number of pairs that are formed and answer[1] is the number of leftover integers in nums after doing the operation as many times as possible.

// Constraints:
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 100

struct Solution;
impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = [0; 101];
        for num in nums {
            freq[num as usize] += 1;
        }

        freq.iter().fold(vec![0, 0], |mut acc, val| {
            acc[0] += val / 2;
            acc[1] += val % 2;
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 2, 1, 3, 2, 2];
        let result = Solution::number_of_pairs(nums);
        assert_eq!(result, vec![3, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1];
        let result = Solution::number_of_pairs(nums);
        assert_eq!(result, vec![1, 0]);
    }

    #[test]
    fn test_3() {
        let nums = vec![0];
        let result = Solution::number_of_pairs(nums);
        assert_eq!(result, vec![0, 1]);
    }
}
