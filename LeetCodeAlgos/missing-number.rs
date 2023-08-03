// Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.


struct Solution {}
impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        nums.sort_unstable();
        for i in 0..n {
            if nums[i as usize] != i {
                return i;
            }
        }
        return n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(Solution::missing_number(vec![3,0,1]), 2);
    }
    
    #[test]
    fn test_missing_number2() {
        assert_eq!(Solution::missing_number(vec![0,1]), 2);
    }
    
    #[test]
    fn test_missing_number3() {
        assert_eq!(Solution::missing_number(vec![9,6,4,2,3,5,7,0,1]), 8);
    }
}