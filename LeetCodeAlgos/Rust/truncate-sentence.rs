// A sentence is a list of words that are separated by a single space with no leading or trailing spaces. Each of the words consists of only uppercase and lowercase English letters (no punctuation).

// For example, "Hello World", "HELLO", and "hello world hello world" are all sentences.
// You are given a sentence s​​​​​​ and an integer k​​​​​​. You want to truncate s​​​​​​ such that it contains only the first k​​​​​​ words. Return s​​​​​​ after truncating it.

// Constraints:
// 1 <= s.length <= 500
// k is in the range [1, the number of words in s].
// s consist of only lowercase and uppercase English letters and spaces.
// The words in s are separated by a single space.
// There are no leading or trailing spaces.

struct Solution;
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split(" ")
            .take(k as usize)
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "Hello how are you Contestant".to_string();
        let k = 4;
        let result = Solution::truncate_sentence(s, k);
        assert_eq!(result, "Hello how are you");
    }

    #[test]
    fn test_2() {
        let s = "What is the solution to this problem".to_string();
        let k = 4;
        let result = Solution::truncate_sentence(s, k);
        assert_eq!(result, "What is the solution");
    }

    #[test]
    fn test_3() {
        let s = "chopper is not a tanuki".to_string();
        let k = 5;
        let result = Solution::truncate_sentence(s, k);
        assert_eq!(result, "chopper is not a tanuki");
    }
}
