// Given an array of integers nums, you start with an initial positive value startValue.

// In each iteration, you calculate the step by step sum of startValue plus elements in nums (from left to right).

// Return the minimum positive value of startValue such that the step by step sum is never less than 1.


struct Solution {}
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = i32::MIN;
        for num in nums.iter() {
            sum += *num;
            if 1 - sum > max {
                max = 1 - sum;
            }
            println!("sum: {}, max: {}", sum, max);
        }
        return if max > 0 { max } else { 1 };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_start_value() {
        assert_eq!(Solution::min_start_value(vec![-3,2,-3,4,2]), 5);
    }

    #[test]
    fn test_min_start_value_2() {
        assert_eq!(Solution::min_start_value(vec![1,2]), 1);
    }

    #[test]
    fn test_min_start_value_3() {
        assert_eq!(Solution::min_start_value(vec![1,-2,-3]), 5);
    }
}