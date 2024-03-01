// You are given a binary string s that contains at least one '1'.

// You have to rearrange the bits in such a way that the resulting binary number is the maximum odd binary number that can be created from this combination.

// Return a string representing the maximum odd binary number that can be created from the given combination.

// Note that the resulting string can have leading zeros.

struct Solution {}
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut ans = String::new();
        let mut odd_bit = false;
        for char in s.chars() {
            if char == '1' {
                if !odd_bit {
                    odd_bit = !odd_bit;
                } else {
                    ans.insert(0, '1');
                }
            } else {
                ans.push(char);
            }
        }
        ans + "1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_odd_binary_number1() {
        assert_eq!(
            Solution::maximum_odd_binary_number("010".to_string()),
            "001".to_string()
        );
    }

    #[test]
    fn test_maximum_odd_binary_number2() {
        assert_eq!(
            Solution::maximum_odd_binary_number("0101".to_string()),
            "1001".to_string()
        );
    }
}
