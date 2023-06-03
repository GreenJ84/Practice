// You are given an array of strings words and a string pref.
// Return the number of strings in words that contain pref as a prefix.
// A prefix of a string s is any leading contiguous substring of s.


struct Solution {}
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.into_iter().filter(|w| w.len() >= pref.len() && w[0..pref.len()] == pref).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_prefix_count() {
        assert_eq!(Solution::prefix_count(vec![String::from("pay"),String::from("attention"),String::from("practice"),String::from("attend")], String::from("at")), 2);
    }

    #[test]
    fn test_prefix_count_2() {
        assert_eq!(Solution::prefix_count(vec![String::from("leetcode"),String::from("win"),String::from("loops"),String::from("success")], String::from("code")), 0);
    }

    #[test]
    fn test_prefix_count_3() {
        let mut vec: Vec<String> = Vec::new();
        for _ in 0..92 {
            vec.push(String::from("sxyjellhlh"));
        }
        assert_eq!(Solution::prefix_count(vec, String::from("sxyjellhlh")), 92);
    }
}