// Given an array of integers nums and an integer k, an element nums[i] is considered good if it is strictly greater than the elements at indices i - k and i + k (if those indices exist). If neither of these indices exists, nums[i] is still considered good.

// Return the sum of all the good elements in the array.

// Constraints:
// 2 <= nums.length <= 100
// 1 <= nums[i] <= 1000
// 1 <= k <= floor(nums.length / 2)

struct Solution;
impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        nums.iter()
            .enumerate()
            .filter_map(|(idx, &v)| {
                let end = idx + k;
                if (idx < k || nums[idx - k] < v) && (end >= n || nums[end] < v) {
                    Some(v)
                } else {
                    None
                }
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 2, 1, 5, 4];
        let k = 2;
        let result = Solution::sum_of_good_numbers(nums, k);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 1];
        let k = 1;
        let result = Solution::sum_of_good_numbers(nums, k);
        assert_eq!(result, 2);
    }
}
