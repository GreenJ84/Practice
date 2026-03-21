// A distinct string is a string that is present only once in an array.

// Given an array of strings arr, and an integer k, return the kth distinct string present in arr. If there are fewer than k distinct strings, return an empty string "".

// Note that the strings are considered in the order in which they appear in the array.

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut freq = HashMap::new();
        for s in arr.iter() {
            *freq.entry(s).or_insert(0) += 1;
        }
        let mut count = 0;
        for s in arr.iter() {
            if freq[s] == 1 {
                count += 1;
                if count == k {
                    return s.clone();
                }
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let arr = vec![
            "d".to_string(),
            "b".to_string(),
            "c".to_string(),
            "b".to_string(),
            "c".to_string(),
            "a".to_string(),
        ];
        let k = 2;
        assert_eq!(Solution::kth_distinct(arr, k), "a".to_string());
    }

    #[test]
    fn test_2() {
        let arr = vec!["aaa".to_string(), "aa".to_string(), "a".to_string()];
        let k = 1;
        assert_eq!(Solution::kth_distinct(arr, k), "aaa".to_string());
    }

    #[test]
    fn test_3() {
        let arr = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let k = 3;
        assert_eq!(Solution::kth_distinct(arr, k), "".to_string());
    }
}
