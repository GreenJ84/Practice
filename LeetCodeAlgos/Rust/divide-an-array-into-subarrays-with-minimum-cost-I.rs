// You are given an array of integers nums of length n.

// The cost of an array is the value of its first element. For example, the cost of [1,2,3] is 1 while the cost of [3,4,1] is 3.

// You need to divide nums into 3 disjoint contiguous subarrays.

// Return the minimum possible sum of the cost of these subarrays.

// Constraints:
// 3 <= n <= 50
// 1 <= nums[i] <= 50

struct Solution;
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let (fir, mut sec, mut third) = (nums[0], 0, 0);
        if nums[1] <= nums[2] {
            (sec, third) = (nums[1], nums[2]);
        } else {
            (sec, third) = (nums[2], nums[1]);
        }

        for i in 3..nums.len() {
            if nums[i] < sec {
                (sec, third) = (nums[i], sec);
            } else if nums[i] < third {
                third = nums[i];
            }
        }

        fir + sec + third
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 12];
        let result = Solution::minimum_cost(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![5, 4, 3];
        let result = Solution::minimum_cost(nums);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_3() {
        let nums = vec![10, 3, 1, 1];
        let result = Solution::minimum_cost(nums);
        assert_eq!(result, 12);
    }
}
