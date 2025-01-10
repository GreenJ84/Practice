// You are given two string arrays words1 and words2.
//
// A string b is a subset of string a if every letter in b occurs in a including multiplicity.
//
// For example, "wrr" is a subset of "warrior" but is not a subset of "world".
// A string a from words1 is universal if for every string b in words2, b is a subset of a.
//
// Return an array of all the universal strings in words1. You may return the answer in any order.
//

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut ans = Vec::<String>::new();
        let mut w2max_freq = HashMap::<char, i32>::new();

        let mut word_freq = HashMap::<char, i32>::new();
        for word in words2 {
            for ch in word.chars(){
                *word_freq.entry(ch).or_insert(0) += 1;
            }

            for (ch, freq) in &word_freq {
                w2max_freq.entry(*ch)
                    .and_modify(|val| *val = *freq.max(val) )
                    .or_insert(*freq);
            }
            word_freq.clear();
        }

        for word1 in words1 {
            for ch in word1.chars() {
                *word_freq.entry(ch).or_insert(0) += 1;
            }

            let mut universal: bool = true;
            for (ch, freq) in &w2max_freq {
                if word_freq.get(ch).unwrap_or(&0) < freq {
                    universal = false;
                    break;
                }
            }
            if universal { ans.push(word1); }
            word_freq.clear();
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            Solution::word_subsets(
                Vec::from(["amazon","apple","facebook","google","leetcode"])
                    .iter().map(|&s| s.to_string()).collect::<Vec<_>>(),
                Vec::from(["e","o"])
                    .iter().map(|&s| s.to_string()).collect::<Vec<_>>()
            ),
            Vec::from(["facebook","google","leetcode"])
                .iter().map(|&s| s.to_string()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            Solution::word_subsets(
                Vec::from(["amazon","apple","facebook","google","leetcode"])
                    .iter().map(|&s| s.to_string()).collect::<Vec<_>>(),
                Vec::from(["l","e"])
                    .iter().map(|&s| s.to_string()).collect::<Vec<_>>()
            ),
            Vec::from(["apple","google","leetcode"])
                .iter().map(|&s| s.to_string()).collect::<Vec<_>>()
        );
    }


    #[test]
    fn test3(){
        assert_eq!(
            Solution::word_subsets(
                Vec::from(["amazon","apple","facebook","google","leetcode"])
                    .iter().map(|&s| s.to_string()).collect::<Vec<_>>(),
                Vec::from(["nzo","o"])
                    .iter().map(|&s| s.to_string()).collect::<Vec<_>>()
            ),
            Vec::from(["amazon"])
                .iter().map(|&s| s.to_string()).collect::<Vec<_>>()
        );
    }
}