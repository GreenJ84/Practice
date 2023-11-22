// Given an integer array nums of positive integers, return the average value of all even integers that are divisible by 3.

// Note that the average of n elements is the sum of the n elements divided by n and rounded down to the nearest integer.


struct Solution {}
impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum: [i32; 2] = [0, 0];
        for n in nums {
            if n % 2 == 0 && n % 3 == 0 {
                sum[0] += n;
                sum[1] += 1;
            }
        }
        return if sum[1] > 0 { sum[0] / sum[1] } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_value() {
        assert_eq!(Solution::average_value(vec![1,3,6,10,12,15]), 9);

    }

    #[test]
    fn test_average_value2() {
        assert_eq!(Solution::average_value(vec![1,2,4,7,10]), 0);
    }
}