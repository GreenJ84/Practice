// Given a 0-indexed integer array nums, find a 0-indexed integer array answer where:

// answer.length == nums.length.
// answer[i] = |leftSum[i] - rightSum[i]|.
// Where:

// leftSum[i] is the sum of elements to the left of the index i in the array nums. If there is no such element, leftSum[i] = 0.
// rightSum[i] is the sum of elements to the right of the index i in the array nums. If there is no such element, rightSum[i] = 0.
// Return the array answer.

struct Solution {}
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left = vec![0; nums.len()];
        let mut right = vec![0; nums.len()];
        let n = nums.len();
        for i in 1..n {
            *left.get_mut(i).unwrap() += *left.get_mut(i - 1).unwrap() + nums.get(i - 1).unwrap();
            *right.get_mut(n - 1 - i).unwrap() += *right.get_mut(n - i).unwrap() + nums.get(n - i).unwrap();
        }
        for i in 0..n {
            *left.get_mut(i).unwrap() = (*left.get_mut(i).unwrap() - *right.get_mut(i).unwrap()).abs();
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_right_difference() {
        assert_eq!(Solution::left_right_difference(vec![10,4,8,3]), vec![15,1,11,22]);
    }

    #[test]
    fn test_left_right_difference_1() {
        assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
    }
}