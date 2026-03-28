// You are given a 0-indexed 2D integer matrix grid of size n * n with values in the range [1, n2]. Each integer appears exactly once except a which appears twice and b which is missing. The task is to find the repeating and missing numbers a and b.

// Return a 0-indexed integer array ans of size 2 where ans[0] equals to a and ans[1] equals to b.

struct Solution;
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut seen = vec![0u8; grid.len() * grid.len() + 1];
        for row in grid {
          for item in row {
            seen[item as usize] += 1;
          }
        }

        let mut ans = vec![0, 0];
        for idx in 0..seen.len() {
          match seen[idx] {
            0 => {
              ans[1] = idx as i32;
            },
            2 => {
              ans[0] = idx as i32;
            },
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
    fn example_1() {
        let grid = vec![vec![1, 3], vec![2, 2]];
        let expected = vec![2, 4];
        assert_eq!(Solution::find_missing_and_repeated_values(grid), expected);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
        let expected = vec![9, 5];
        assert_eq!(Solution::find_missing_and_repeated_values(grid), expected);
    }
}

// Example 1:

// Input: grid = [[1,3],[2,2]]
// Output: [2,4]
// Explanation: Number 2 is repeated and number 4 is missing so the answer is [2,4].
// Example 2:

// Input: grid = [[9,1,7],[8,9,2],[3,4,6]]
// Output: [9,5]
// Explanation: Number 9 is repeated and number 5 is missing so the answer is [9,5].