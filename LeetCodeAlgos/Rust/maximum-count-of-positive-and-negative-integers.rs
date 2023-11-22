// Given an array nums sorted in non-decreasing order, return the maximum between the number of positive integers and the number of negative integers.
// In other words, if the number of positive integers in nums is pos and the number of negative integers is neg, then return the maximum of pos and neg.
// Note that 0 is neither positive nor negative.


struct Solution {}
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();
        if min > &0 || max < &0 { return nums.len() as i32; }
        let mut count = [0, 0];
        for n in nums {
            if n > 0 { count[0] += 1; }
            if n < 0 { count[1] += 1; }
        }
        return if count[0] > count[1] { count[0] } else { count[1] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_count() {
        assert_eq!(Solution::maximum_count(vec![-2,-1,-1,1,2,3]), 3);
    }

    #[test]
    fn test_maximum_count_2() {
        assert_eq!(Solution::maximum_count(vec![-3,-2,-1,0,0,1,2]), 3);
    }

    #[test]
    fn test_maximum_count_3() {
        assert_eq!(Solution::maximum_count(vec![5,20,66,1314]), 4);
    }
}