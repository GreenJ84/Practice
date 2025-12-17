// Given two strings a and b, return the length of the longest uncommon subsequence between a and b. If no such uncommon subsequence exists, return -1.

// An uncommon subsequence between two strings is a string that is a subsequence of exactly one of them.

struct Solution;
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            return -1;
        }
        a.len().max(b.len()) as i32
    }

    pub fn find_lu_slength1(a: String, b: String) -> i32 {
        return if a == b {
            -1
        } else {
            a.len().max(b.len()) as i32
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::find_lu_slength("aba".to_string(), "cdc".to_string()),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::find_lu_slength("aaa".to_string(), "bbb".to_string()),
            3
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()),
            -1
        );
    }

    #[test]
    fn test_single_character_different() {
        assert_eq!(
            Solution::find_lu_slength("a".to_string(), "b".to_string()),
            1
        );
    }

    #[test]
    fn test_single_character_same() {
        assert_eq!(
            Solution::find_lu_slength("a".to_string(), "a".to_string()),
            -1
        );
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(
            Solution::find_lu_slength("ab".to_string(), "abcd".to_string()),
            4
        );
    }

    #[test]
    fn test_completely_different_strings() {
        assert_eq!(
            Solution::find_lu_slength("xyz".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn test_max_length_strings() {
        let a = "a".repeat(100);
        let b = "b".repeat(100);
        assert_eq!(Solution::find_lu_slength(a, b), 100);
    }
}
