// Given an array nums of integers, return how many of them contain an even number of digits.

struct Solution {}
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for n in nums {
            if n.to_string().len() % 2 == 0 {
                ans += 1;
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_numbers() {
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    }

    #[test]
    fn test_find_numbers_2() {
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}