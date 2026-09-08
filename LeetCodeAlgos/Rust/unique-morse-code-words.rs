// International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes, as follows:

// 'a' maps to ".-",
// 'b' maps to "-...",
// 'c' maps to "-.-.", and so on.
// For convenience, the full table for the 26 letters of the English alphabet is given below:

// [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
// Given an array of strings words where each word can be written as a concatenation of the Morse code of each letter.

// For example, "cab" can be written as "-.-..--...", which is the concatenation of "-.-.", ".-", and "-...". We will call such a concatenation the transformation of a word.
// Return the number of different transformations among all words we have.

// Constraints:
// 1 <= words.length <= 100
// 1 <= words[i].length <= 12
// words[i] consists of lowercase English letters.

struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let codes = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        let mut unique = HashSet::<String>::new();

        for word in words {
            let morse = word.chars().map(|c| {
                codes[(c as u8 - b'a') as usize]
            })
            .collect::<Vec<&str>>()
            .join("");
            unique.insert(morse);
        }
        unique.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["gin".to_string(), "zen".to_string(), "gig".to_string(), "msg".to_string()];
        let result = Solution::unique_morse_representations(words);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let words = vec!["a".to_string()];
        let result = Solution::unique_morse_representations(words);
        assert_eq!(result, 1);
    }
}
