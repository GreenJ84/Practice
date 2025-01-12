// You are given a string s.
//
// You can perform the following process on s any number of times:
//
// Choose an index i in the string such that there is at least one character to the left of index i that is equal to s[i], and at least one character to the right that is also equal to s[i].
// Delete the closest character to the left of index i that is equal to s[i].
// Delete the closest character to the right of index i that is equal to s[i].
// Return the minimum length of the final string s that you can achieve.
//

struct Solution {}
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut freq: [i32; 26] = [0; 26];
        for ch in s.chars(){
            let idx = ch as usize - 'a' as usize;
            freq[idx] += if freq[idx] == 2 { -1 } else { 1 };
        }
        freq.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            Solution::minimum_length(String::from("abaacbcbb")),
            5
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            Solution::minimum_length(String::from("aa")),
            2
        );
    }
}