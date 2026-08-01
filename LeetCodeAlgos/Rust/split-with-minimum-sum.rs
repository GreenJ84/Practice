// Given a positive integer num, split it into two non-negative integers num1 and num2 such that:

// The concatenation of num1 and num2 is a permutation of num.
// In other words, the sum of the number of occurrences of each digit in num1 and num2 is equal to the number of occurrences of that digit in num.
// num1 and num2 can contain leading zeros.
// Return the minimum possible sum of num1 and num2.

// Notes:

// It is guaranteed that num does not contain any leading zeros.
// The order of occurrence of the digits in num1 and num2 may differ from the order of occurrence of num.
// Constraints:
// 10 <= num <= 10^9

struct Solution;
impl Solution {
    pub fn split_num(mut num: i32) -> i32 {
        let mut nums = vec![];
        while num > 0 {
            nums.push(num % 10);
            num /= 10;
        }
        nums.sort_unstable();

        let (mut left, mut right) = (nums[0], nums[1]);
        for i in 2..nums.len() {
            if i % 2 == 0 {
                left *= 10;
                left += nums[i];
            } else {
                right *= 10;
                right += nums[i];
            }
        }
        left + right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 4325;
        let result = Solution::split_num(num);
        assert_eq!(result, 59);
    }

    #[test]
    fn test_2() {
        let num = 687;
        let result = Solution::split_num(num);
        assert_eq!(result, 75);
    }

    #[test]
    fn test_3() {
        let num = 10;
        let result = Solution::split_num(num);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_4() {
        let num = 111000;
        let result = Solution::split_num(num);
        assert_eq!(result, 12);
    }
}
