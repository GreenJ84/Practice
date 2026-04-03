// You are given an integer array nums containing positive integers. We define a function encrypt such that encrypt(x) replaces every digit in x with the largest digit in x. For example, encrypt(523) = 555 and encrypt(213) = 333.

// Return the sum of encrypted elements.

struct Solution;
impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for num in nums.into_iter() {
            sum += Self::encrypt(num);
        }
        sum
    }

    fn encrypt(mut num: i32) -> i32 {
        let mut digits = 0;
        let mut max_digit = 0;
        while num > 0 {
            max_digit = max_digit.max(num % 10);
            num /= 10;
            digits += 1;
        }
        num = max_digit;
        for _ in 1..digits {
            num *= 10;
            num += max_digit;
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::sum_of_encrypted_int(nums), 6);
    }

    #[test]
    fn test2() {
        let nums = vec![10, 21, 31];
        assert_eq!(Solution::sum_of_encrypted_int(nums), 66);
    }
}

// Example 1:

// Input: nums = [1,2,3]

// Output: 6

// Explanation: The encrypted elements are [1,2,3]. The sum of encrypted elements is 1 + 2 + 3 == 6.

// Example 2:

// Input: nums = [10,21,31]

// Output: 66

// Explanation: The encrypted elements are [11,22,33]. The sum of encrypted elements is 11 + 22 + 33 == 66.
