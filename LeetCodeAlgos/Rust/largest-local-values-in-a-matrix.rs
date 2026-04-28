// You are given an n x n integer matrix grid.

// Generate an integer matrix maxLocal of size (n - 2) x (n - 2) such that:

// maxLocal[i][j] is equal to the largest value of the 3 x 3 matrix in grid centered around row i + 1 and column j + 1.
// In other words, we want to find the largest value in every contiguous 3 x 3 matrix in grid.

// Return the generated matrix.

// Constraints:
// - n == grid.length == grid[i].length
// - 3 <= n <= 100
// - 1 <= grid[i][j] <= 100

struct Solution;
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
      let n = grid.len();
      (1..n-1).map(|row| {
        (1..n-1).map(|col| {
          let mut max_val = grid[row][col];
          for dr in 0..=2 {
            for dc in 0..=2 {
              max_val = max_val.max(grid[row+dr-1][col+dc-1]);
            }
          }
          max_val
        })
        .collect::<Vec<i32>>()
      })
      .collect::<Vec<Vec<i32>>>()
    }

    pub fn _largest_local1(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
      let n = grid.len();
      let mut ans = vec![vec![0; n-2]; n-2];

      for r in 1..n-1 {
        for c in 1..n-1 {
          let mut max_val = grid[r][c];
          for dr in 0..=2 {
            for dc in 0..=2 {
              if dr == 1 && dc == 1 {
                continue;
              }
              max_val = max_val.max(grid[r+dr-1][c+dc-1]);
            }
          }
          ans[r-1][c-1] = max_val;
        }
      }

      ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![
            vec![9, 9, 8, 1],
            vec![5, 6, 2, 6],
            vec![8, 2, 6, 4],
            vec![6, 2, 2, 2]
        ];
        let expected = vec![
            vec![9, 9],
            vec![8, 6]
        ];
        assert_eq!(Solution::largest_local(grid), expected);
    }

    #[test]
    fn test_2() {
        let grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 2, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1]
        ];
        let expected = vec![
            vec![2, 2, 2],
            vec![2, 2, 2],
            vec![2, 2, 2]
        ];
        assert_eq!(Solution::largest_local(grid), expected);
    }

    #[test]
    fn test_3() {
        let grid = vec![
            vec![3, 1, 1, 1, 3],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 2, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![3, 1, 1, 1, 3]
        ];
        let expected = vec![
            vec![3, 2, 3],
            vec![2, 2, 2],
            vec![3, 2, 3]
        ];
        assert_eq!(Solution::largest_local(grid), expected);
    }
}

// Example 1:


// Input: grid = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
// Output: [[9,9],[8,6]]
// Explanation: The diagram above shows the original matrix and the generated matrix.
// Notice that each value in the generated matrix corresponds to the largest value of a contiguous 3 x 3 matrix in grid.
// Example 2:


// Input: grid = [[1,1,1,1,1],[1,1,1,1,1],[1,1,2,1,1],[1,1,1,1,1],[1,1,1,1,1]]
// Output: [[2,2,2],[2,2,2],[2,2,2]]
// Explanation: Notice that the 2 is contained within every contiguous 3 x 3 matrix in grid.