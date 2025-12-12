// Given an integer num, return a string of its base 7 representation.

struct Solution;
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut n = num.abs();

        let mut base_7 = String::with_capacity(10);
        while n > 0 {
            base_7.push_str(&format!("{}", n % 7));
            n /= 7;
        }
        if num < 0 {
            base_7.push('-')
        }
        base_7.chars().rev().collect()
    }

    pub fn convert_to_base7_1(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let neg = num < 0;
        let mut num = num.abs() as u32;

        let mut base_7 = String::new();
        loop {
            base_7.push_str(&format!("{}", num % 7));
            num /= 7;
            if num == 0 {
                if neg {
                    base_7.push('-')
                }
                break base_7.chars().rev().collect();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_positive() {
        assert_eq!(Solution::convert_to_base7(100), "202");
    }

    #[test]
    fn test_example_negative() {
        assert_eq!(Solution::convert_to_base7(-7), "-10");
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::convert_to_base7(0), "0");
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::convert_to_base7(1), "1");
    }

    #[test]
    fn test_negative_one() {
        assert_eq!(Solution::convert_to_base7(-1), "-1");
    }

    #[test]
    fn test_base_transition() {
        assert_eq!(Solution::convert_to_base7(7), "10");
        assert_eq!(Solution::convert_to_base7(8), "11");
    }

    #[test]
    fn test_square_of_base() {
        assert_eq!(Solution::convert_to_base7(49), "100");
    }

    #[test]
    fn test_within_constraints_positive() {
        assert_eq!(Solution::convert_to_base7(107), "212");
    }

    #[test]
    fn test_within_constraints_negative() {
        assert_eq!(Solution::convert_to_base7(-107), "-212");
    }

    #[test]
    fn test_upper_bound() {
        assert_eq!(Solution::convert_to_base7(10_000_000), "150666343");
    }

    #[test]
    fn test_lower_bound() {
        assert_eq!(Solution::convert_to_base7(-10_000_000), "-150666343");
    }
}
