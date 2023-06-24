// Given a string s consisting of words and spaces, return the length of the last word in the string.

// A word is a maximal substring consisting of non-space characters only.


struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn test_length_of_last_word_2() {
        assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
    }

    #[test]
    fn test_length_of_last_word_3() {
        assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}