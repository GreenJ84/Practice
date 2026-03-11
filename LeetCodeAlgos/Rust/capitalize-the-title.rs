// You are given a string title consisting of one or more words separated by a single space, where each word consists of English letters. Capitalize the string by changing the capitalization of each word such that:

// If the length of the word is 1 or 2 letters, change all letters to lowercase.
// Otherwise, change the first letter to uppercase and the remaining letters to lowercase.
// Return the capitalized title.

struct Solution;
impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title.split(' ')
          .map(|seg| seg.to_lowercase() )
          .map(|seg| if seg.len() < 3 {
            seg
          } else {
            let mut temp = seg.chars().collect::<Vec<char>>();
            temp[0] = temp[0].to_ascii_uppercase();
            temp.into_iter().collect::<String>()
          })
          .collect::<Vec<String>>()
          .join(" ")
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let title = "capiTalIze tHe titLe".to_string();
    let expected = "Capitalize The Title".to_string();
    assert_eq!(Solution::capitalize_title(title), expected);
  }

  #[test]
  fn test_2() {
    let title = "First leTTeR of EACH Word".to_string();
    let expected = "First Letter of Each Word".to_string();
    assert_eq!(Solution::capitalize_title(title), expected);
  }

  #[test]
  fn test_3() {
    let title = "i lOve leetcode".to_string();
    let expected = "i Love Leetcode".to_string();
    assert_eq!(Solution::capitalize_title(title), expected);
  }
}