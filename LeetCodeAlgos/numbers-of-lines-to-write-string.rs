// You are given a string s of lowercase English letters and an array widths denoting how many pixels wide each lowercase English letter is. Specifically, widths[0] is the width of 'a', widths[1] is the width of 'b', and so on.

// You are trying to write s across several lines, where each line is no longer than 100 pixels. Starting at the beginning of s, write as many letters on the first line such that the total width does not exceed 100 pixels. Then, from where you stopped in s, continue writing as many letters as you can on the second line. Continue this process until you have written all of s.

// Return an array result of length 2 where:

// result[0] is the total number of lines.
// result[1] is the width of the last line in pixels.

struct Solution {}
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut curr: i32 = 0;
        let mut lines: i32 = 1;
        for letter in s.chars(){
            let ascii: u8 = letter as u8 - 97;
            if curr + widths[ascii as usize] <= 100 {
                curr += widths[ascii as usize];
            } else {
                curr = 0;
                lines += 1;
                curr += widths[ascii as usize];
            };
        }
        Vec::from([lines, curr])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_lines() {
        assert_eq!(Solution::number_of_lines(vec![10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], String::from("abcdefghijklmnopqrstuvwxyz")), vec![3, 60]);
    }

    #[test]
    fn test_number_of_lines_2() {
        assert_eq!(Solution::number_of_lines(vec![4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], String::from("bbbcccdddaaa")), vec![2, 4]);
    }
}