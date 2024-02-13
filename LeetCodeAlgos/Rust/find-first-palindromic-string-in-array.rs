// Given an array of strings words, return the first palindromic string in the array. If there is no such string, return an empty string "".

// A string is palindromic if it reads the same forward and backward.

struct Solution {}
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words.iter() {
            if Self::is_palindrome(word) {
                return word.to_string();
            }
        }
        String::new()
    }

    fn is_palindrome(word: &str) -> bool {
        let string_array = word.chars().collect::<Vec<char>>();
        let mut start: usize = 0;
        let mut end: usize = string_array.len() - 1;
        while start <= end {
            if string_array[start]!= string_array[end] { return false; }
            start += 1;
            if end != 0 { end -= 1; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_palindrome() {
        assert_eq!(Solution::first_palindrome(vec!["abc".to_string(),"car".to_string(),"ada".to_string(),"racecar".to_string(),"cool".to_string()]), "ada".to_string());
    }

    #[test]
    fn test_first_palindrome_2() {
        assert_eq!(Solution::first_palindrome(vec!["notapalindrome".to_string(),"racecar".to_string()]), "racecar".to_string());
    }

    #[test]
    fn test_first_palindrome_3() {
        assert_eq!(Solution::first_palindrome(vec!["def".to_string(),"ghi".to_string()]), "".to_string());
    }
}