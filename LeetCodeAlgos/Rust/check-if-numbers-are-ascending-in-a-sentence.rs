// A sentence is a list of tokens separated by a single space with no leading or trailing spaces. Every token is either a positive number consisting of digits 0-9 with no leading zeros, or a word consisting of lowercase English letters.

// For example, "a puppy has 2 eyes 4 legs" is a sentence with seven tokens: "2" and "4" are numbers and the other tokens such as "puppy" are words.
// Given a string s representing a sentence, you need to check if all the numbers in s are strictly increasing from left to right (i.e., other than the last number, each number is strictly smaller than the number on its right in s).

// Return true if so, or false otherwise.

struct Solution;
impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = 0;
        for token in s.split(' ') {
            if let Ok(num) = token.parse::<i32>() {
                if num <= prev {
                    return false;
                }
                prev = num;
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::are_numbers_ascending("1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::are_numbers_ascending("hello world 5 x 5".to_string()),
            false
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::are_numbers_ascending("sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()),
            false
        );
    }
}