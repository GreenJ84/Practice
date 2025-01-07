// Given an array of string words, return all strings in words that is a substring of another word. You can return the answer in any order.
//
// A substring is a contiguous sequence of characters within a string

use std::collections::*;

struct Solution {}
impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        let n = words.len();
        words.sort_by(|a, b| b.len().cmp(&a.len()));

        let mut set: HashSet<String> = HashSet::new();

        for sup in 0..n {
            for sub in (sup + 1)..n{
                let sub = &words[sub];
                if !set.contains(sub) && words[sup].contains(sub){
                    set.insert(sub.clone());
                }
            }
        }

        set.into_iter().collect::<Vec<String>>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_vec(mut vec: Vec<String>) -> Vec<String> {
        vec.sort(); // Sort the vector lexicographically
        vec
    }


    #[test]
    fn test1(){
        assert_eq!(
            sorted_vec(Solution::string_matching(Vec::from(["mass".to_string(),"as".to_string(),"hero".to_string(),"superhero".to_string()]))),
            sorted_vec(Vec::from(["as".to_string(),"hero".to_string()]))
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            sorted_vec(Solution::string_matching(Vec::from(["leetcode".to_string(),"et".to_string(),"code".to_string()]))),
            sorted_vec(Vec::from(["et".to_string(),"code".to_string()]))
        );
    }

    #[test]
    fn test3(){
        assert_eq!(
            sorted_vec(Solution::string_matching(Vec::from(["blue".to_string(),"green".to_string(),"bu".to_string()]))),
            sorted_vec(Vec::<String>::new())
        );
    }
}