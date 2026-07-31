// Given an integer array nums, partition it into two (contiguous) subarrays left and right so that:

// Every element in left is less than or equal to every element in right.
// left and right are non-empty.
// left has the smallest possible size.
// Return the length of left after such a partitioning.

// Test cases are generated such that partitioning exists.

// Constraints:
// 2 <= nums.length <= 105
// 0 <= nums[i] <= 106
// There is at least one valid answer for the given input.

struct Solution;
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut window_max = nums[0];
        let mut partition = (0usize, window_max);
        for idx in 1..n {
            if nums[idx] < partition.1 {
                partition = (idx, window_max);
            }
            if nums[idx] > window_max {
                window_max = nums[idx];
            }
        }
        for idx in ((partition.0 + 1)..n).rev() {
            if nums[idx] < partition.1 {
                return idx as i32 + 1;
            }
        }
        partition.0 as i32 + 1
    }

    // Failed test case #3
    pub fn partition_disjoint1(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut window_max = nums[0];
        let mut min_max = (0usize, window_max);
        for idx in 1..n {
            if nums[idx] < nums[min_max.0] {
                min_max = (idx, window_max);
            } else if nums[idx] > window_max {
                window_max = nums[idx];
            }
        }

        for idx in ((min_max.0 + 1)..n).rev() {
            if nums[idx] < min_max.1 {
                return idx as i32 + 1;
            }
        }
        min_max.0 as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![5, 0, 3, 8, 6];
        let result = Solution::partition_disjoint(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 0, 6, 12];
        let result = Solution::partition_disjoint(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let nums = vec![
            29, 33, 6, 4, 42, 0, 10, 22, 62, 16, 46, 75, 100, 67, 70, 74, 87, 69, 73, 88,
        ];
        let result = Solution::partition_disjoint(nums);
        assert_eq!(result, 11);
    }
}
