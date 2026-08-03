// A string originalText is encoded using a slanted transposition cipher to a string encodedText with the help of a matrix having a fixed number of rows rows.

// originalText is placed first in a top-left to bottom-right manner.

// The blue cells are filled first, followed by the red cells, then the yellow cells, and so on, until we reach the end of originalText. The arrow indicates the order in which the cells are filled. All empty cells are filled with ' '. The number of columns is chosen such that the rightmost column will not be empty after filling in originalText.

// encodedText is then formed by appending all characters of the matrix in a row-wise fashion.

// The characters in the blue cells are appended first to encodedText, then the red cells, and so on, and finally the yellow cells. The arrow indicates the order in which the cells are accessed.

// For example, if originalText = "cipher" and rows = 3, then we encode it in the following manner:

// The blue arrows depict how originalText is placed in the matrix, and the red arrows denote the order in which encodedText is formed. In the above example, encodedText = "ch ie pr".

// Given the encoded string encodedText and number of rows rows, return the original string originalText.

// Note: originalText does not have any trailing spaces ' '. The test cases are generated such that there is only one possible originalText.

// Constraints:
// 0 <= encodedText.length <= 10^6
// encodedText consists of lowercase English letters and ' ' only.
// encodedText is a valid encoding of some originalText that does not have trailing spaces.
// 1 <= rows <= 1000
// The testcases are generated such that there is only one possible originalText.

struct Solution;
impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let cols = encoded_text.len() / rows as usize;
        let rows = rows as usize;
        if rows == 1 || cols == 0 {
            return encoded_text;
        }

        let mut result = String::new();
        for c in 0..cols {
            for r in 0..rows {
                let idx = c + r * cols + r;
                if idx >= encoded_text.len() {
                    break;
                }
                result.push_str(&encoded_text[idx..idx + 1]);
            }
        }
        result.trim_end().into()
    }

    pub fn decode_ciphertext1(encoded_text: String, rows: i32) -> String {
        if rows == 1 {
            return encoded_text;
        }
        let cols = encoded_text.len() / rows as usize;
        let grid = encoded_text
            .chars()
            .collect::<Vec<char>>()
            .chunks(cols)
            .map(|ch| ch.into())
            .collect::<Vec<Vec<char>>>();
        let rows = grid.len();

        let mut result = String::new();
        for c in 0..cols {
            for r in 0..rows {
                if r + c < cols {
                    result.push(grid[r][c + r]);
                }
            }
        }
        result.trim_end().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let encoded_text = "ch   ie   pr".to_string();
        let rows = 3;
        let result = Solution::decode_ciphertext(encoded_text, rows);
        assert_eq!(result, "cipher");
    }

    #[test]
    fn test_2() {
        let encoded_text = "iveo    eed   l te   olc".to_string();
        let rows = 4;
        let result = Solution::decode_ciphertext(encoded_text, rows);
        assert_eq!(result, "i love leetcode");
    }

    #[test]
    fn test_3() {
        let encoded_text = "coding".to_string();
        let rows = 1;
        let result = Solution::decode_ciphertext(encoded_text, rows);
        assert_eq!(result, "coding");
    }
}
