// Given an integer array arr, return the length of a maximum size turbulent subarray of arr.

// A subarray is turbulent if the comparison sign flips between each adjacent pair of elements in the subarray.

// More formally, a subarray [arr[i], arr[i + 1], ..., arr[j]] of arr is said to be turbulent if and only if:

// For i <= k < j:
// arr[k] > arr[k + 1] when k is odd, and
// arr[k] < arr[k + 1] when k is even.
// Or, for i <= k < j:
// arr[k] > arr[k + 1] when k is even, and
// arr[k] < arr[k + 1] when k is odd.

// Constraints:
// 1 <= arr.length <= 4 * 10^4
// 0 <= arr[i] <= 10^9

struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let (mut max1, mut max2) = (1, 1);
        let (mut left1, mut left2) = (0, 0);

        for right in 1..arr.len() {
            let k = &arr[right - 1];
            let (is_even, comparison) = ((right - 1) % 2 == 0, k.cmp(&arr[right]));
            match (is_even, comparison) {
                (_, Ordering::Equal) => {
                    left1 = right;
                    left2 = right;
                }
                // First variant odd/even schema
                (false, Ordering::Greater) => {
                    max1 = max1.max((right - left1 + 1) as i32);
                    left2 = right;
                }
                (true, Ordering::Less) => {
                    max1 = max1.max((right - left1 + 1) as i32);
                    left2 = right;
                }
                // second variant odd/even schema
                (false, Ordering::Less) => {
                    max2 = max2.max((right - left2 + 1) as i32);
                    left1 = right;
                }
                (true, Ordering::Greater) => {
                    max2 = max2.max((right - left2 + 1) as i32);
                    left1 = right;
                }
            }
        }
        max1.max(max2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
    }
}

// Example 1:

// Input: arr = [9,4,2,10,7,8,8,1,9]
// Output: 5
// Explanation: arr[1] > arr[2] < arr[3] > arr[4] < arr[5]
// Example 2:

// Input: arr = [4,8,12,16]
// Output: 2
// Example 3:

// Input: arr = [100]
// Output: 1
