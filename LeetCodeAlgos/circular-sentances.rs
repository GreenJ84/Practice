// A sentence is a list of words that are separated by a single space with no leading or trailing spaces.

// For example, "Hello World", "HELLO", "hello world hello world" are all sentences.
// Words consist of only uppercase and lowercase English letters. Uppercase and lowercase English letters are considered different.

// A sentence is circular if:

// The last character of a word is equal to the first character of the next word.
// The last character of the last word is equal to the first character of the first word.
// For example, "leetcode exercises sound delightful", "eetcode", "leetcode eats soul" are all circular sentences. However, "Leetcode is cool", "happy Leetcode", "Leetcode" and "I like Leetcode" are not circular sentences.

// Given a string sentence, return true if it is circular. Otherwise, return false.


struct Solution {}
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut begin: Option<char> = Some('a');
        let mut end: Option<char> = Some('z');

        for (idx, c) in sentence.split(" ").collect::<Vec<&str>>().iter().enumerate() {
            let mut ch = c.chars();
            let len = ch.clone().count();

            if idx == 0 { 
                begin = ch.nth(0);
                if len == 1 {
                    end = begin.clone();
                    continue;
                }
            } else {
                let first = ch.nth(0);
                if first != end {
                    return false;
                }
                if len == 1 {
                    end = first;
                    continue;
                }
            }
            end = ch.last();
        }
        if end == begin { return true } else { return false };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_circular_sentence() {
        assert_eq!(Solution::is_circular_sentence("leetcode exercises sound delightful".to_string()), true);
    }

    #[test]
    fn test_is_not_circular_sentence() {
        assert_eq!(Solution::is_circular_sentence("eetcode".to_string()), true);
    }

    #[test]
    fn test_is_circular_sentence_2() {
        assert_eq!(Solution::is_circular_sentence("Leetcode is cool".to_string()), false);
    }

    #[test]
    fn test_is_circular_sentence_3() {
        assert_eq!(Solution::is_circular_sentence("IuTiUtGGsNydmacGduehPPGksKQyT TmOraUbCcQdnZUCpGCYtGp p pG GCcRvZDRawqGKOiBSLwjIDOjdhnHiisfddYoeHqxOqkUvOEyI".to_string()), true);
    }

    #[test]
    fn test_is_circular_sentence_4() {
        assert_eq!(Solution::is_circular_sentence("Q Qx xxdelQrdIHiOAMKzJfKwSmgOmXFvyLQUSpSdIfeIdMiyRPhSQ".to_string()), true);
    }
}