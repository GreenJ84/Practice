// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 || strs[0].is_empty() {
            return strs[0].to_owned();
        }
        let mut prefix = String::from(strs[0].to_owned());
        for i in 1..strs.len() {
            if strs[i].is_empty(){
                return "".to_string();
            }
            for j in 0..strs[i].len() {
                // If over prefix, prefix remains longest
                if j == prefix.len(){
                    break;
                }
                // When not equal, prefix ends at the first mismatch
                if strs[i].as_bytes()[j]!= prefix.as_bytes()[j] {
                    prefix = prefix[..j].to_string();
                    break;
                }
                // If we finsih the entire word, its the new longest
                if j == strs[i].len() - 1{
                    prefix = strs[i].to_owned();
                }
            }
        }
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl".to_string());
    }

    #[test]
    fn test_longest_common_prefix2() {
        assert_eq!(Solution::longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]), "".to_string());
    }

    #[test]
    fn test_longest_common_prefix3() {
        assert_eq!(Solution::longest_common_prefix(vec!["abab".to_string(),"aba".to_string(),"".to_string()]), "".to_string());
    }
}