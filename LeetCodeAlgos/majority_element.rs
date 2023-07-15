// Given an array nums of size n, return the majority element.
// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.



struct Solution {}
impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn test_majority_element2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn test_majority_element3() {
        assert_eq!(Solution::majority_element(vec![1]), 1);
    }
}

