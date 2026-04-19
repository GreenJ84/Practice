// You are given a string num representing a large integer. An integer is good if it meets the following conditions:

// It is a substring of num with length 3.
// It consists of only one unique digit.
// Return the maximum good integer as a string or an empty string "" if no such integer exists.

// Note:

// A substring is a contiguous sequence of characters within a string.
// There may be leading zeroes in num or a good integer.

struct Solution;
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
      let num = num.chars().collect::<Vec<char>>();

      let mut largest = None;
      let mut window = 1;
      for i in 1..num.len() {
        if num[i] == num[i - 1] {
            window += 1;
            if window == 3 && (largest.is_none() || num[i] > largest.unwrap()) {
              largest = Some(num[i]);
            }
        } else {
            window = 1;
        }
      }

      match largest {
        Some(ch) => ch.to_string().repeat(3),
        None => String::new(),
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = "6777133339".to_string();
        let expected = "777".to_string();
        assert_eq!(Solution::largest_good_integer(num), expected);
    }

    #[test]
    fn test_2() {
        let num = "2300019".to_string();
        let expected = "000".to_string();
        assert_eq!(Solution::largest_good_integer(num), expected);
    }

    #[test]
    fn test_3() {
        let num = "42352338".to_string();
        let expected = "".to_string();
        assert_eq!(Solution::largest_good_integer(num), expected);
    }
}
// Example 1:

// Input: num = "6777133339"
// Output: "777"
// Explanation: There are two distinct good integers: "777" and "333".
// "777" is the largest, so we return "777".
// Example 2:

// Input: num = "2300019"
// Output: "000"
// Explanation: "000" is the only good integer.
// Example 3:

// Input: num = "42352338"
// Output: ""
// Explanation: No substring of length 3 consists of only one unique digit. Therefore, there are no good integers.