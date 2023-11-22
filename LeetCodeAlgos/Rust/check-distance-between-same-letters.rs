// You are given a 0-indexed string s consisting of only lowercase English letters, where each letter in s appears exactly twice. You are also given a 0-indexed integer array distance of length 26.

// Each letter in the alphabet is numbered from 0 to 25 (i.e. 'a' -> 0, 'b' -> 1, 'c' -> 2, ... , 'z' -> 25).

// In a well-spaced string, the number of letters between the two occurrences of the ith letter is distance[i]. If the ith letter does not appear in s, then distance[i] can be ignored.

// Return true if s is a well-spaced string, otherwise return false.

use std::collections::HashMap;


struct Solution {}
impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut seen: HashMap<char, u32> = HashMap::new();
        for (idx, val) in s.chars().collect::<Vec<char>>().iter().enumerate() {
            print!("{}", val);
            match seen.get(val) {
                Some(&j) => {
                    if idx as u32 - j - 1 != distance[(val.to_digit(36).unwrap() - 10) as usize] as u32{
                        return false;
                    }
                },
                None => {
                    seen.insert(*val, idx as u32);
                }
            }
        }
        true
    }
}

fn main() {
    for a in vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'].iter() {
        print!("{}", a);
        print!("{:?}", a.to_digit(36).unwrap() - 9);
    }
    // Solution::check_distances(String::from("aa"), vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test() {
        assert_eq!(Solution::check_distances(String::from("abaccb"), vec![1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::check_distances(String::from("aa"), vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]), false);
    }
}