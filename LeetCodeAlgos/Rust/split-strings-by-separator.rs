// Given an array of strings words and a character separator, split each string in words by separator.

// Return an array of strings containing the new strings formed after the splits, excluding empty strings.

// Notes

// separator is used to determine where the split should occur, but it is not included as part of the resulting strings.
// A split may result in more than two strings.
// The resulting strings must maintain the same order as they were initially given.

struct Solution;
impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words.into_iter()
          .map(|w| {
              w.split(separator)
                  .filter(|s| !s.is_empty())
                  .map(str::to_string)
                  .collect()
          })
          .flatten()
          .collect()
    }

      pub fn _split_words_by_separator1(words: Vec<String>, separator: char) -> Vec<String> {
        words.into_iter()
          .flat_map(|w| {
              w.split(separator)
                  .filter(|s| !s.is_empty())
                  .map(str::to_string)
                  .collect::<Vec<String>>()
          })
          .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["one.two.three".to_string(), "four.five".to_string(), "six".to_string()];
        let separator = '.';
        let expected = vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string()];
        assert_eq!(Solution::split_words_by_separator(words, separator), expected);
    }

    #[test]
    fn test_2() {
        let words = vec!["$easy$".to_string(), "$problem$".to_string()];
        let separator = '$';
        let expected = vec!["easy".to_string(), "problem".to_string()];
        assert_eq!(Solution::split_words_by_separator(words, separator), expected);
    }

    #[test]
    fn test_3() {
        let words = vec!["|||".to_string()];
        let separator = '|';
        let expected: Vec<String> = vec![];
        assert_eq!(Solution::split_words_by_separator(words, separator), expected);
    }
}

// Example 1:

// Input: words = ["one.two.three","four.five","six"], separator = "."
// Output: ["one","two","three","four","five","six"]
// Explanation: In this example we split as follows:

// "one.two.three" splits into "one", "two", "three"
// "four.five" splits into "four", "five"
// "six" splits into "six"

// Hence, the resulting array is ["one","two","three","four","five","six"].
// Example 2:

// Input: words = ["$easy$","$problem$"], separator = "$"
// Output: ["easy","problem"]
// Explanation: In this example we split as follows:

// "$easy$" splits into "easy" (excluding empty strings)
// "$problem$" splits into "problem" (excluding empty strings)

// Hence, the resulting array is ["easy","problem"].
// Example 3:

// Input: words = ["|||"], separator = "|"
// Output: []
// Explanation: In this example the resulting split of "|||" will contain only empty strings, so we return an empty array []. 
