// Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.

// Note that the same word in the dictionary may be reused multiple times in the segmentation.


struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n: usize = s.len();
        let mut dp: [bool; 1000] = [false; 1000];
        dp[0] = true;

        for i in 1..(n + 1){
            for j in 0..i{
                if dp[j] && word_dict.contains(&s[j..i].to_string()){
                    dp[i] = true;
                    break
                }
            }
        }
        return dp[n];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert_eq!(Solution::word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]), true);
    }

    #[test]
    fn test_word_break2() {
        assert_eq!(Solution::word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]), true);
    }

    #[test]
    fn test_word_break3() {
        assert_eq!(Solution::word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), false);
    }
}
