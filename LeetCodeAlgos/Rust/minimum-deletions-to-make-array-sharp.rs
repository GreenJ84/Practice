// You are given an integer array nums.

// You may delete any number of elements from nums. The remaining elements keep their original order.

// The resulting array a is sharp if the bitwise XOR of every pair of adjacent elements is non-zero. That is, a[i] XOR a[i + 1] != 0 for every index i such that 0 <= i < a.length - 1. An array with at most one element is sharp.

// Return the minimum number of deletions needed to make nums sharp.

// Bitwise XOR compares the corresponding bits of two integers. A bit in the result is 1 if the corresponding bits differ, and 0 otherwise.

// Constraints:
// 2 <= nums.length <= 100
// 1 <= nums[i] <= 100

struct Solution;
impl Solution {
    pub fn min_deletions(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .filter(|(i, &n)| {
                println!("{}, {} {}", i, n, *i != 0 && nums[i - 1] == n);
                *i != 0 && nums[i - 1] == n
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 2, 2, 2, 3];
        let result = Solution::min_deletions(nums);
        assert_eq!(result, 3);
    }
    #[test]
    fn test2() {
        let nums = vec![4, 4, 9, 4];
        let result = Solution::min_deletions(nums);
        assert_eq!(result, 1);
    }
}
