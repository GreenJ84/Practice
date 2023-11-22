// Given a string s, reverse only all the vowels in the string and return it.

// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.

struct Solution {}


impl Solution {
    const VOWELS: &'static [char] = &['a', 'e','i', 'o', 'u'];
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels: Vec<char> = Vec::new();

        for ch in s.chars() {
            if Self::VOWELS.contains( &ch.to_ascii_lowercase() ) {
                vowels.push(ch);
            }
        }

        let mut res = String::new();

        for ch in s.chars(){
            if Self::VOWELS.contains( &ch.to_ascii_lowercase() ) {
                res.push( vowels.pop().unwrap() );
            } else {
                res.push(ch);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_vowels() {
        assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
    }

    #[test]
    fn test_reverse_vowels_2() {
        assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
        
    }

    #[test]
    fn test_reverse_vowels_3() {
        assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa".to_string());
        
    }
}