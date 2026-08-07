// You are given a personal information string s, representing either an email address or a phone number. Return the masked personal information using the below rules.

// Email address:

//  An email address is:
//  - A name consisting of at least two uppercase and lowercase English letters, followed by
//  - The '@' symbol, followed by
//  - The domain consisting of uppercase and lowercase English letters with a dot '.' somewhere in the middle (not the first or last character).

//  To mask an email:
//  - The uppercase letters in the name and domain must be converted to lowercase letters.
//  - The middle letters of the name (i.e., all but the first and last letters) must be replaced by 5 asterisks "*****".

//  Phone number:

//    A phone number is formatted as follows:
//    - The phone number contains 10-13 digits.
//    - The last 10 digits make up the local number.
//    - The remaining 0-3 digits, in the beginning, make up the country code.
//    - Separation characters from the set {'+', '-', '(', ')', ' '} separate the above digits in some way.

//    To mask a phone number:
//    - Remove all separation characters.
//    - The masked phone number should have the form:
//      - "***-***-XXXX" if the country code has 0 digits.
//      - "+*-***-***-XXXX" if the country code has 1 digit.
//      - "+**-***-***-XXXX" if the country code has 2 digits.
//      - "+***-***-***-XXXX" if the country code has 3 digits.
//    - "XXXX" is the last 4 digits of the local number.

struct Solution;
impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.contains("@") {
            let s = s.to_lowercase();
            let [name, domain, ..] = s.split('@').collect::<Vec<&str>>()[..] else {
                return String::new();
            };
            let mut name = name.chars();
            let fir = name.next().unwrap();
            let sec = name.last().unwrap();
            return format!("{}*****{}@{}", fir, sec, domain);
        }

        let s = s.chars().filter(|c| c.is_digit(10)).collect::<String>();
        let n = s.len();
        let last = &s[n - 4..];
        match n {
            10 => {
                format!("***-***-{}", last)
            }
            _ => {
                format!("+{}-***-***-{}", "*".repeat(n - 10), last)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("LeetCode@LeetCode.com");
        let result = Solution::mask_pii(s);
        assert_eq!(result, "l*****e@leetcode.com");
    }

    #[test]
    fn test_2() {
        let s = String::from("AB@qq.com");
        let result = Solution::mask_pii(s);
        assert_eq!(result, "a*****b@qq.com");
    }

    #[test]
    fn test_3() {
        let s = String::from("1(234)567-890");
        let result = Solution::mask_pii(s);
        assert_eq!(result, "***-***-7890");
    }
}
