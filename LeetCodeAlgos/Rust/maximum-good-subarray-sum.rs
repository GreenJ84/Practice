// You are given an array nums of length n and a positive integer k.

// A subarray of nums is called good if the absolute difference between its first and last element is exactly k, in other words, the subarray nums[i..j] is good if |nums[i] - nums[j]| == k.

// Return the maximum sum of a good subarray of nums. If there are no good subarrays, return 0.

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut min_at_value = HashMap::<i32, i64>::new();

        let mut max_sum = i64::MIN;
        let mut sum = 0i64;
        for idx in 0..nums.len() {
            let num = nums[idx];
            let mut current_sum = i64::MIN;
            // Check K over
            if let Some(start) = min_at_value.get(&(num - k)) {
                // Calculate subarray sum
                // Update max_sum
                current_sum = current_sum.max(sum - start + num as i64);
            }
            // Check K under
            if let Some(start) = min_at_value.get(&(num + k)) {
                // Calculate subarray sum

                // Update max_sum
                current_sum = current_sum.max(sum - start + num as i64);
            }
            // Insert prefix_sum
            min_at_value
                .entry(num)
                .and_modify(|e| *e = (*e).min(sum))
                .or_insert(sum);

            sum += num as i64;
            max_sum = max_sum.max(current_sum);
        }

        if max_sum == i64::MIN {
            return 0;
        }
        max_sum
    }

    pub fn maximum_subarray_sum1(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let sums = {
            let mut v = vec![0i64; n + 1];
            for idx in 0..n {
                v[idx + 1] = nums[idx] as i64 + v[idx];
            }
            v
        };

        let mut max_sum = i64::MIN;
        for i in 0..n - 1 {
            for j in i + 1..n {
                if (nums[i] - nums[j]).abs() != k {
                    continue;
                }
                let window = if i == 0 {
                    sums[j + 1]
                } else {
                    sums[j + 1] - sums[i]
                };
                max_sum = max_sum.max(window);
            }
        }
        if max_sum == i64::MIN {
            return 0;
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![1, 2, 3, 4, 5, 6], 1),
            11
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::maximum_subarray_sum(vec![-1, 3, 2, 4, 5], 3), 11);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::maximum_subarray_sum(vec![-1, -2, -3, -4], 2), -6);
    }

    #[test]
    fn test_single_good_subarray() {
        assert_eq!(Solution::maximum_subarray_sum(vec![5, 10], 5), 15);
    }

    #[test]
    fn test_no_good_subarray() {
        assert_eq!(Solution::maximum_subarray_sum(vec![1, 2, 3, 4], 10), 0);
    }

    #[test]
    fn test_all_same_elements() {
        assert_eq!(Solution::maximum_subarray_sum(vec![5, 5, 5, 5], 1), 0);
    }

    #[test]
    fn test_minimum_length() {
        assert_eq!(Solution::maximum_subarray_sum(vec![1, 2], 1), 3);
    }

    #[test]
    fn test_negative_sum_good_subarray() {
        assert_eq!(Solution::maximum_subarray_sum(vec![-5, -10, -3], 2), -18);
    }

    #[test]
    fn test_multiple_good_subarrays() {
        assert_eq!(Solution::maximum_subarray_sum(vec![1, 5, 0, 5, 10], 5), 20);
    }
}
