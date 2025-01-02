// Given a string s of zeros and ones, return the maximum score after splitting the string into two non-empty substrings (i.e. left substring and right substring).

// The score after splitting a string is the number of zeros in the left substring plus the number of ones in the right substring.

struct Solution {}
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut max_score = 0;

        let mut left_zeros = 0;
        let mut right_ones = s.clone().chars().filter(|c| *c == '1').count() as i32;

        for (idx, ch) in s.char_indices() {
            if idx == s.len() - 1 { return max_score }
            if ch == '0' {
                left_zeros += 1;
            } else {
                right_ones -= 1;
            }
            max_score = max_score.max(left_zeros + right_ones);
        }
        max_score
    }
    //
    // pub fn max_score(s: String) -> i32 {
    //     let mut max_score = 0;
    //     let mut left = 0;
    //     let mut right = s.clone().chars().filter(|c| *c == '1').count() as i32;
    //     let mut it = s.chars().peekable();
    //     while let Some(ch)  = it.next() {
    //         if let Some(_) = it.peek(){
    //             if ch == '0' {
    //                 left += 1;
    //             } else {
    //                 right -= 1;
    //             }
    //             max_score = max_score.max(left + right);
    //         }
    //     }
    //     max_score
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
    }

    #[test]
    fn test_max_score_2() {
        assert_eq!(Solution::max_score("00111".to_string()), 5);
    }

    #[test]
    fn test_max_score_3() {
        assert_eq!(Solution::max_score("1111".to_string()), 3);
    }

    #[test]
    fn test_max_score_4() {
        assert_eq!(Solution::max_score("00".to_string()), 1);
    }

    #[test]
    fn test_max_score_5() {
        assert_eq!(Solution::max_score("11".to_string()), 1);
    }
}