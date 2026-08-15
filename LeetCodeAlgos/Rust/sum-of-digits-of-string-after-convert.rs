// You are given a string s consisting of lowercase English letters, and an integer k. Your task is to convert the string into an integer by a special process, and then transform it by summing its digits repeatedly k times. More specifically, perform the following steps:

// Convert s into an integer by replacing each letter with its position in the alphabet (i.e. replace 'a' with 1, 'b' with 2, ..., 'z' with 26).
// Transform the integer by replacing it with the sum of its digits.
// Repeat the transform operation (step 2) k times in total.
// For example, if s = "zbax" and k = 2, then the resulting integer would be 8 by the following operations:

// Convert: "zbax" ➝ "(26)(2)(1)(24)" ➝ "262124" ➝ 262124
// Transform #1: 262124 ➝ 2 + 6 + 2 + 1 + 2 + 4 ➝ 17
// Transform #2: 17 ➝ 1 + 7 ➝ 8
// Return the resulting integer after performing the operations described above.

// Constraints:
// 1 <= s.length <= 100
// 1 <= k <= 10
// s consists of lowercase English letters.

struct Solution;
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut sum = 0;
        for ch in s.chars() {
            let digit = ch as i32 - 96;
            sum += digit % 10 + digit / 10;
        }
        for _ in 1..k {
            if sum < 10 {
                return sum;
            }
            let mut digits = 0;
            while sum > 0 {
                digits += sum % 10;
                sum /= 10;
            }
            sum = digits;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "iiii".to_string();
        let k = 1;
        let result = Solution::get_lucky(s, k);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_2() {
        let s = "leetcode".to_string();
        let k = 2;
        let result = Solution::get_lucky(s, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_3() {
        let s = "zbax".to_string();
        let k = 2;
        let result = Solution::get_lucky(s, k);
        assert_eq!(result, 8);
    }
}
