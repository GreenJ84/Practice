// Given a string s, return the length of the longest substring between two equal characters, excluding the two characters. If there is no such substring return -1.

// A substring is a contiguous sequence of characters within a string.

struct Solution;
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
      let mut last_alpha: [i32; 26] = [-1; 26];
      let mut ans: i32 = -1;
      for (idx, char) in s.chars().enumerate() {
        let last_letter: i32 = last_alpha[char as usize - 97];
        match (last_letter >= 0, (idx as i32) - last_letter - 1 > ans){
          (false, _) => { last_alpha[char as usize - 97] = idx as i32; },
          (true, true) => { ans = (idx as i32) - last_letter - 1; },
          _ => {}
        }
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
      Solution::max_length_between_equal_characters(String::from("aa")),
      0
    );
  }

  #[test]
  fn test2(){
    assert_eq!(
      Solution::max_length_between_equal_characters(String::from("abca")),
      2
    );
  }

  #[test]
  fn test3(){
    assert_eq!(
      Solution::max_length_between_equal_characters(String::from("cbzxy")),
      -1
    );
  }

  #[test]
  fn test4(){
    assert_eq!(
      Solution::max_length_between_equal_characters(String::from("mgntdygtxrvxjnwksqhxuxtrv")),
      18
    );
  }
}