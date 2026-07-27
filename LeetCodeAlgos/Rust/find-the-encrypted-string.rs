// You are given a string s and an integer k. Encrypt the string using the following algorithm:

// For each character c in s, replace c with the kth character after c in the string (in a cyclic manner).
// Return the encrypted string.

// Constraints:
// 1 <= s.length <= 100
// 1 <= k <= 104
// s consists only of lowercase English letters.

struct Solution;
impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
      let n = s.len();
      let k = k as usize % n;
        let mut ans = String::new();
        for i in 0..n {
          let enc = (i + k) % n;
          ans.push_str( &s[enc..(enc + 1)] );
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "dart".to_string();
        let k = 3;
        let result = Solution::get_encrypted_string(s, k);
        assert_eq!(result, "tdar");
    }

    #[test]
    fn test_2() {
        let s = "aaa".to_string();
        let k = 1;
        let result = Solution::get_encrypted_string(s, k);
        assert_eq!(result, "aaa");
    }
}