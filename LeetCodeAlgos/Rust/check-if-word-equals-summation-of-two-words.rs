// The letter value of a letter is its position in the alphabet starting from 0 (i.e. 'a' -> 0, 'b' -> 1, 'c' -> 2, etc.).

// The numerical value of some string of lowercase English letters s is the concatenation of the letter values of each letter in s, which is then converted into an integer.

// For example, if s = "acb", we concatenate each letter's letter value, resulting in "021". After converting it, we get 21.
// You are given three strings firstWord, secondWord, and targetWord, each consisting of lowercase English letters 'a' through 'j' inclusive.

// Return true if the summation of the numerical values of firstWord and secondWord equals the numerical value of targetWord, or false otherwise.

// Constraints:
// 1 <= firstWord.length, secondWord.length, targetWord.length <= 8
// firstWord, secondWord, and targetWord consist of lowercase English letters from 'a' to 'j' inclusive.

struct Solution;
impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let first_number = first_word.chars().fold(0i32, |acc, c| acc * 10 + ((c as u8 - b'a') as i32));
        let second_number = second_word.chars().fold(0i32, |acc, c| acc * 10 + ((c as u8 - b'a') as i32));

        first_number + second_number == target_word.chars().fold(0, |acc, c| acc * 10 + ((c as u8 - b'a') as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let first_word = "acb".to_string();
        let second_word = "cba".to_string();
        let target_word = "cdb".to_string();
        let result = Solution::is_sum_equal(first_word, second_word, target_word);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let first_word = "aaa".to_string();
        let second_word = "a".to_string();
        let target_word = "aab".to_string();
        let result = Solution::is_sum_equal(first_word, second_word, target_word);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let first_word = "aaa".to_string();
        let second_word = "a".to_string();
        let target_word = "aaaa".to_string();
        let result = Solution::is_sum_equal(first_word, second_word, target_word);
        assert_eq!(result, true);
    }

    #[test]
    fn test_4() {
        let first_word = "fcjhfcg".to_string();
        let second_word = "iaabhbh".to_string();
        let target_word = "bdcjjced".to_string();
        let result = Solution::is_sum_equal(first_word, second_word, target_word);
        assert_eq!(result, true);
    }
}
