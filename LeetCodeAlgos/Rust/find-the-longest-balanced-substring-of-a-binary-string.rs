// You are given a binary string s consisting only of zeroes and ones.

// A substring of s is considered balanced if all zeroes are before ones and the number of zeroes is equal to the number of ones inside the substring. Notice that the empty substring is considered a balanced substring.

// Return the length of the longest balanced substring of s.

// A substring is a contiguous sequence of characters within a string.

struct Solution;
impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut longest = 0;

        let (mut zeros, mut run, mut flip) = (0, 0, false);
        for ch in s.chars() {
            match ch {
                '0' => {
                    // If already looking for 1's and find 0, check substring and reset
                    if flip {
                        longest = longest.max(run);
                        (zeros, run, flip) = (0, 0, false);
                    }
                    // Add zeros to a applicable run
                    zeros += 1;
                }
                _ => {
                    // Sanity check, no left side currently
                    if zeros == 0 {
                        continue;
                    }
                    // We have gone from 0's to 1's in valid substring
                    if !flip {
                        flip = true;
                    }
                    // Add the pair, 0/1 to the run
                    run += 2;
                    // Remove parable 0 we just used
                    zeros -= 1;
                    // If no more pairs, this is longest possible
                    if zeros == 0 {
                        longest = longest.max(run);
                        (run, flip) = (0, false);
                    }
                }
            }
        }
        longest.max(run)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("01000111".to_string()),
            6
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("00111".to_string()),
            4
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("111".to_string()),
            0
        );
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("0000".to_string()),
            0
        );
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("01".to_string()),
            2
        );
    }

    #[test]
    fn test_multiple_balanced() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("0101".to_string()),
            2
        );
    }

    #[test]
    fn test_balanced_at_end() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("11110011".to_string()),
            4
        );
    }

    #[test]
    fn test_entire_string_balanced() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("0011".to_string()),
            4
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("0".to_string()),
            0
        );
    }

        #[test]
    fn test_early_end() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("001".to_string()),
            2
        );
    }
}
