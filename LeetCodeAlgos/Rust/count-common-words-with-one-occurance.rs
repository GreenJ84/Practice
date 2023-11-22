// Given two string arrays words1 and words2, return the number of strings that appear exactly once in each of the two arrays.

use std::collections::HashMap;
use std::cmp::max;

struct Solution {}
impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map: HashMap<String, i32> = HashMap::new();

        words1.iter().for_each(|word| *map.entry(word.clone()).or_insert(0) += 3);

        words2.iter().for_each(|word| *map.entry(word.clone()).or_insert(0) -= 2);

        map.into_values().filter(|x| *x == 1).collect::<Vec<_>>().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_count_common_words() {
        let mut words1: Vec<String> = Vec::new();
        let mut words2: Vec<String> = Vec::new();
        for i in ["leetcode","is","amazing","as","is"]{
            words1.push(i.to_string());
        }
        for j in ["amazing","leetcode","is"]{
            words2.push(j.to_string());
        }
        assert_eq!(Solution::count_words(words1, words2), 2);
    }

    #[test]
    fn test_count_common_words_2() {
        let mut words1: Vec<String> = Vec::new();
        let mut words2: Vec<String> = Vec::new();
        for i in ["b","bb","bbb"]{
            words1.push(i.to_string());
        }
        for j in ["a","aa","aaa"]{
            words2.push(j.to_string());
        }
        assert_eq!(Solution::count_words(words1, words2), 0);
    }

    #[test]
    fn test_count_common_words_3() {
        let mut words1: Vec<String> = Vec::new();
        let mut words2: Vec<String> = Vec::new();
        for i in ["a","ab"]{
            words1.push(i.to_string());
        }
        for j in ["a","a","a","ab"]{
            words2.push(j.to_string());
        }
        assert_eq!(Solution::count_words(words1, words2), 1);
    }
}