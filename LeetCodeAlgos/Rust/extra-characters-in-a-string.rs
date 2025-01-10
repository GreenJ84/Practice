// You are given a 0-indexed string s and a dictionary of words dictionary. You have to break s into one or more non-overlapping substrings such that each substring is present in dictionary. There may be some extra characters in s which are not present in any of the substrings.
//
// Return the minimum number of extra characters left over if you break up s optimally.

struct Solution {}

// def minExtraCharacters(s, dictionary):
    // n = len(s)
    // dp = [float('inf')] * (n + 1)  # Initialize DP array
    // dp[0] = 0  # Base case: no extra characters for empty string
    //
    // # Convert dictionary to a set for O(1) lookups
    // word_set = set(dictionary)
    //
    // for i in range(1, n + 1):
        // # Assume the entire substring s[0:i] is extra
        // dp[i] = dp[i - 1] + 1
        //
        // # Check all possible substrings ending at i
        // for j in range(i):
            // substring = s[j:i]
            // if substring in word_set:
                // dp[i] = min(dp[i], dp[j])
    // return dp[n]

use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let mut dp = vec![i32::max_value(); n+1];
        dp[0] = 0;

        let mut dict = HashSet::<String>::with_capacity(dictionary.len());
        dict.extend(dictionary.into_iter());
        for i in 1..n+1 {
            dp[i] = dp[i - 1] + 1;

            for j in 0..i{
                if dict.contains(&s[j..i]){
                    dp[i] = dp[i].min(dp[j]);
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            Solution::min_extra_char(
                "leetscode".to_string(),
                Vec::from(["leet".to_string(),"code".to_string(),"leetcode".to_string()])
            ),
            1
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            Solution::min_extra_char(
                "sayhelloworld".to_string(),
                Vec::from(["hello".to_string(),"world".to_string()])
            ),
            3
        );
    }
}