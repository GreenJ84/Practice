// You are given a 0-indexed string blocks of length n, where blocks[i] is either 'W' or 'B', representing the color of the ith block. The characters 'W' and 'B' denote the colors white and black, respectively.

// You are also given an integer k, which is the desired number of consecutive black blocks.

// In one operation, you can recolor a white block such that it becomes a black block.

// Return the minimum number of operations needed such that there is at least one occurrence of k consecutive black blocks.

// Constraints:
// n == blocks.length
// 1 <= n <= 100
// blocks[i] is either 'W' or 'B'.
// 1 <= k <= n

struct Solution;
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let k = k as usize;
        let mut window = 0;
        for idx in 0..k {
          if blocks[idx] == b'W' {
            window += 1;
          }
        }

        let mut ans = window;
        for idx in k..blocks.len() {
          if ans == 0 {
            return ans;
          }
          if blocks[idx] == b'W' {
            window += 1;
          }
          if blocks[idx - k] == b'W' {
            window -= 1;
          }
          ans = ans.min(window);
        }
        ans
    }

  pub fn minimum_recolors1(blocks: String, k: i32) -> i32 {
        let blocks: Vec<char> = blocks.chars().collect();
        let k = k as usize;
        let mut window = 0;
        for idx in 0..k {
          if blocks[idx] == 'W' {
            window += 1;
          }
        }

        let mut ans = window;
        for idx in k..blocks.len() {
          if ans == 0 {
            return ans;
          }
          if blocks[idx] == 'W' {
            window += 1;
          }
          if blocks[idx - k] == 'W' {
            window -= 1;
          }
          ans = ans.min(window);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let blocks = String::from("WBBWWBBWBW");
    let k = 7;
    assert_eq!(Solution::minimum_recolors(blocks, k), 3);
  }

  #[test]
  fn test_2() {
    let blocks = String::from("WBWBBBW");
    let k = 2;
    assert_eq!(Solution::minimum_recolors(blocks, k), 0);
  }
}

// Example 1:

// Input: blocks = "WBBWWBBWBW", k = 7
// Output: 3
// Explanation:
// One way to achieve 7 consecutive black blocks is to recolor the 0th, 3rd, and 4th blocks
// so that blocks = "BBBBBBBWBW". 
// It can be shown that there is no way to achieve 7 consecutive black blocks in less than 3 operations.
// Therefore, we return 3.
// Example 2:

// Input: blocks = "WBWBBBW", k = 2
// Output: 0
// Explanation:
// No changes need to be made, since 2 consecutive black blocks already exist.
// Therefore, we return 0.
