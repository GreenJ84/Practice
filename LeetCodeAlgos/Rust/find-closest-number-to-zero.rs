// Given an integer array nums of size n, return the number with the value closest to 0 in nums. If there are multiple answers, return the number with the largest value.

// Constraints:
// - 1 <= n <= 1000
// - -105 <= nums[i] <= 105

struct Solution;
impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        *nums
            .iter()
            .min_by(|a, b| a.abs().cmp(&b.abs())
              .then_with(|| b.cmp(&a))
            )
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-4, -2, 1, 4, 8];
        assert_eq!(Solution::find_closest_number(nums), 1);
    }

    #[test]
    fn test2() {
        let nums = vec![2, -1, 1];
        assert_eq!(Solution::find_closest_number(nums), 1);
    }
}

// Example 1:

// Input: nums = [-4,-2,1,4,8]
// Output: 1
// Explanation:
// The distance from -4 to 0 is |-4| = 4.
// The distance from -2 to 0 is |-2| = 2.
// The distance from 1 to 0 is |1| = 1.
// The distance from 4 to 0 is |4| = 4.
// The distance from 8 to 0 is |8| = 8.
// Thus, the closest number to 0 in the array is 1.
// Example 2:

// Input: nums = [2,-1,1]
// Output: 1
// Explanation: 1 and -1 are both the closest numbers to 0, so 1 being larger is returned.
