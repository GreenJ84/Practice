// You are given an array of n strings strs, all of the same length.

// The strings can be arranged such that there is one on each line, making a grid.

// - For example, strs = ["abc", "bce", "cae"] can be arranged as follows:
//  [ a, b, c ]
//  [ b, c, e ]
//  [ c, a, e ]

// You want to delete the columns that are not sorted lexicographically. In the above example (0-indexed), columns 0 ('a', 'b', 'c') and 2 ('c', 'e', 'e') are sorted, while column 1 ('b', 'c', 'a') is not, so you would delete column 1.

// Return the number of columns that you will delete.

struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
      let grid = strs.iter()
        .map(|st| st.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

      let mut ans = 0;
      for col in 0..grid[0].len() {
        for row in 1..grid.len() {
          if grid[row][col] < grid[row - 1][col] {
            ans += 1;
            break;
          }
        }
      }
      ans
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let strs = vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 1);
  }

  #[test]
  fn test_example_2() {
    let strs = vec!["a".to_string(), "b".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 0);
  }

  #[test]
  fn test_example_3() {
    let strs = vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 3);
  }

  #[test]
  fn test_all_sorted() {
    let strs = vec!["abc".to_string(), "bcd".to_string(), "cde".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 0);
  }

  #[test]
  fn test_all_unsorted() {
    let strs = vec!["cba".to_string(), "bcb".to_string(), "aba".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 3);
  }

  #[test]
  fn test_single_string() {
    let strs = vec!["abc".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 0);
  }

  #[test]
  fn test_duplicate_characters() {
    let strs = vec!["aaa".to_string(), "aaa".to_string(), "aaa".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 0);
  }

  #[test]
  fn test_mixed_sorted_unsorted() {
    let strs = vec!["abc".to_string(), "bce".to_string(), "cae".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 1);
  }
}
