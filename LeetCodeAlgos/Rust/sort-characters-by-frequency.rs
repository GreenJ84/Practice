// Given a string s, sort it in decreasing order based on the frequency of the characters. The frequency of a character is the number of times it appears in the string.

// Return the sorted string. If there are multiple answers, return any of them.

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut track: HashMap<char, i32> = HashMap::new();
        s.chars().for_each(|c| *track.entry(c).or_insert(0) += 1);

        let mut frequencies: Vec<(char, i32)> = track.into_iter().collect::<Vec<(char, i32)>>();
        frequencies.sort_by(|a, b| b.1.cmp(&a.1));

        let mut ans = String::new();
        frequencies
            .into_iter()
            .for_each(|(ch, num)| ans.push_str(&ch.to_string().repeat(num as usize)));

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort1() {
        let result = Solution::frequency_sort("tree".to_string());
        assert!(
            result == "eert".to_string() || result == "eetr".to_string()
        );
    }

    #[test]
    fn test_frequency_sort2() {
        let result = Solution::frequency_sort("cccaaa".to_string());
        assert!(
            result == "cccaaa".to_string() || result == "aaaccc".to_string()
        );
    }

    #[test]
    fn test_frequency_sort3() {
        let result = Solution::frequency_sort("Aabb".to_string());
        assert!(
            result == "bbAa".to_string() || result == "bbaA".to_string()
        );
    }
}
