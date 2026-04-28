// You are given an integer num. You know that Bob will sneakily remap one of the 10 possible digits (0 to 9) to another digit.

// Return the difference between the maximum and minimum values Bob can make by remapping exactly one digit in num.

// Notes:

// When Bob remaps a digit d1 to another digit d2, Bob replaces all occurrences of d1 in num with d2.
// Bob can remap a digit to itself, in which case num does not change.
// Bob can remap different digits for obtaining minimum and maximum values respectively.
// The resulting number after remapping can contain leading zeroes.

// Constraints:
// 1 <= num <= 108

struct Solution;
impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        // Convert the number to a vector of digits
        let digits = {
            let mut digits = Vec::new();
            let mut n = num;
            while n > 0 {
                digits.push(n % 10);
                n /= 10;
            }
            digits
        };


        // Find remap idx for maximum and minimum values
        let (mut max_d, mut min_d) = (None, None);
        for i in (0..digits.len()).rev() {
            if digits[i] != 9 && max_d.is_none() {
                max_d = Some(digits[i]);
            }
            if digits[i] != 0 && min_d.is_none() {
                min_d = Some(digits[i]);
            }
            if max_d.is_some() && min_d.is_some() {
                break;
            }
        }

        let mut ans = 0;
        for i in 0..digits.len() {
            let ma = if let Some(d) = max_d {
                if digits[i] == d {
                    9
                } else {
                    digits[i]
                }
            } else {
                digits[i]
            };

            let mi = if let Some(d) = min_d {
                if digits[i] == d {
                    0
                } else {
                    digits[i]
                }
            } else {
                digits[i]
            };
            ans += (ma - mi) * (10i32).pow(i as u32);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 11891;
        assert_eq!(Solution::min_max_difference(num), 99009);
    }

    #[test]
    fn test_2() {
        let num = 90;
        assert_eq!(Solution::min_max_difference(num), 99);
    }

    #[test]
    fn test_3() {
        let num = 12345;
        assert_eq!(Solution::min_max_difference(num), 90000);
    }

    #[test]
    fn test_4() {
        let num = 90000;
        assert_eq!(Solution::min_max_difference(num), 99999);
    }

    #[test]
    fn test_5() {
        let num = 99999;
        assert_eq!(Solution::min_max_difference(num), 99999);
    }
}
