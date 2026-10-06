// You are given a sorted integer array nums and an integer k.

// Return an array such that each distinct element appears at most k times, while preserving the relative order of the elements in nums.

// Note: If a distinct element appears at least k times, then it must appear exactly k times in the resulting array.

// Constraints:
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100
// nums is sorted in non-decreasing order.
// 1 <= k <= nums.length

// Follow-up:
// Can you solve this in-place using O(1) extra space?
// Note that the space used for returning or resizing the result does not count toward the space complexity mentioned above, as some languages do not support in-place resizing.

struct Solution;
impl Solution {
    pub fn limit_occurrences(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        if n <= k {
            return nums;
        }

        let mut place = 0usize;
        let mut run = 0usize;
        let mut prev = 0i32;
        for i in 0..n {
            if nums[i] == prev {
                run += 1;
            } else {
                prev = nums[i];
                run = 1;
            }

            if run <= k {
                nums[place] = nums[i];
                place += 1;
            }
        }
        nums.truncate(place);
        nums
    }

    pub fn limit_occurrences1(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() <= k as usize {
            return nums;
        }
        let mut i = 1usize;
        let mut place = 1usize;
        let mut prev = nums[0];
        let mut run = 1i32;
        while i < nums.len() {
            match (run == k, nums[i] == prev) {
                (true, true) => {
                    place -= 1;
                }
                (true, false) => {
                    nums[place] = nums[i];
                    prev = nums[i];
                    run = 1;
                }
                (false, true) => {
                    run += 1;
                    if place != i {
                        nums[place] = nums[i];
                    }
                }
                _ => {
                    run = 1;
                    prev = nums[i];
                    if place != i {
                        nums[place] = nums[i];
                    }
                }
            }
            place += 1;
            i += 1;
        }
        nums.truncate(place);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = Solution::limit_occurrences(nums, k);
        assert_eq!(result, vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3];
        let k = 1;
        let result = Solution::limit_occurrences(nums, k);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 1, 1, 1, 1];
        let k = 3;
        let result = Solution::limit_occurrences(nums, k);
        assert_eq!(result, vec![1, 1, 1]);
    }
}
