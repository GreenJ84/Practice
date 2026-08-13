// You are given an array of equal-length strings words. Assume that the length of each string is n.

// Each string words[i] can be converted into a difference integer array difference[i] of length n - 1 where difference[i][j] = words[i][j+1] - words[i][j] where 0 <= j <= n - 2. Note that the difference between two letters is the difference between their positions in the alphabet i.e. the position of 'a' is 0, 'b' is 1, and 'z' is 25.

// For example, for the string "acb", the difference integer array is [2 - 0, 1 - 2] = [2, -1].
// All the strings in words have the same difference integer array, except one. You should find that string.

// Return the string in words that has different difference integer array.

// Constraints:
// 3 <= words.length <= 100
// n == words[i].length
// 2 <= n <= 20
// words[i] consists of lowercase English letters.

struct Solution;
impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut first_diff = vec![0i8; words[0].len() - 1]; // First diff
        let mut non_match = false; // Second words doesn't matching first_diff

        for (i, word) in words.iter().enumerate() {
            let mut last = None; // Last seen char in a word

            for (j, ch) in word.chars().enumerate() {
                if last.is_none() {
                    last = Some(ch);
                    continue;
                }
                let diff = ch as i8 - last.unwrap() as i8;
                last = Some(ch);

                if i == 0 {
                    // Populate first_diff
                    first_diff[j - 1] = diff;
                } else if first_diff[j - 1] != diff {
                    // Doesn't match first_diff
                    if i == 1 {
                        non_match = true;
                        break;
                    }
                    return match non_match {
                        true => words[0].to_owned(),
                        false => words[i].to_owned(),
                    };
                }
            }
            // If second word doesn't match first_diff but another does
            if i > 1 && non_match {
                return words[1].to_owned();
            }
        }
        return words[0].to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["adc".to_string(), "wzy".to_string(), "abc".to_string()];
        assert_eq!(Solution::odd_string(words), "abc".to_string());
    }

    #[test]
    fn test_2() {
        let words = vec![
            "aaa".to_string(),
            "bob".to_string(),
            "ccc".to_string(),
            "ddd".to_string(),
        ];
        assert_eq!(Solution::odd_string(words), "bob".to_string());
    }
}
