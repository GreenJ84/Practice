// Given an integer array nums, return the third distinct maximum number in this array. If the third maximum does not exist, return the maximum number.

struct Solution {}
impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a,b| b.cmp(a));
        nums.dedup();
        return if nums.len() > 2 { nums[2] } else { nums[0]}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(Solution::third_max(vec![3,2,1]), 1);
    }
    
    #[test]
    fn test_third_max2() {
        assert_eq!(Solution::third_max(vec![1,2]), 2);
    }
    
    #[test]
    fn test_third_max3() {
        assert_eq!(Solution::third_max(vec![2,2,3,1]), 1);
    }

    #[test]
    fn test_third_max4() {
        assert_eq!(Solution::third_max(vec![1,2,2,5,3,5]), 2);
    }
}