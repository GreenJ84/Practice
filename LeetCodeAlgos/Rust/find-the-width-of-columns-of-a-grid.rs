// You are given a 0-indexed m x n integer matrix grid. The width of a column is the maximum length of its integers.

// For example, if grid = [[-10], [3], [12]], the width of the only column is 3 since -10 is of length 3.
// Return an integer array ans of size n where ans[i] is the width of the ith column.

// The length of an integer x with len digits is equal to len if x is non-negative, and len + 1 otherwise.


struct Solution;
impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
      let mut ans = Vec::new();

      fn check_width(mut num: i32) -> i32 {
        let mut width = if num > 0 { 0 } else { 1 };
        let mut num = num.abs();
        while num != 0 {
          num /= 10;
          width += 1;
        }
        width
      };

      for col in 0..grid[0].len() {
        let mut max_col_width = 0;

        for row in 0..grid.len() {
          let col_width = check_width(grid[row][col]);
          if col_width > max_col_width {
            max_col_width = col_width;
          }
        }

        ans.push(max_col_width);
      }
      ans
    }

    pub fn find_column_width1(grid: Vec<Vec<i32>>) -> Vec<i32> {
      let (m, n) = (grid.len(), grid[0].len());
      let mut ans = Vec::with_capacity(n);

      for col in 0..n {
        let mut max_col_width = 0;

        for row in 0..m {
          let col_width = grid[row][col].to_string().len() as i32;
          if col_width > max_col_width {
            max_col_width = col_width;
          }
        }

        ans.push(max_col_width);
      }
      ans
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let grid = vec![vec![1], vec![22], vec![333]];
    assert_eq!(Solution::find_column_width(grid), vec![3]);
  }

  #[test]
  fn test_example_2() {
    let grid = vec![
      vec![-15, 1, 3],
      vec![15, 7, 12],
      vec![5, 6, -2],
    ];
    assert_eq!(Solution::find_column_width(grid), vec![3, 1, 2]);
  }

  #[test]
  fn test_single_element() {
    let grid = vec![vec![0]];
    assert_eq!(Solution::find_column_width(grid), vec![1]);
  }

  #[test]
  fn test_negative_numbers() {
    let grid = vec![vec![-10], vec![-999], vec![3]];
    assert_eq!(Solution::find_column_width(grid), vec![4]);
  }

  #[test]
  fn test_multiple_columns() {
    let grid = vec![vec![1, -1, 100], vec![999, 0, -999]];
    assert_eq!(Solution::find_column_width(grid), vec![3, 2, 4]);
  }

  #[test]
  fn test_large_negative_number() {
    let grid = vec![vec![-1000000000], vec![1000000000]];
    assert_eq!(Solution::find_column_width(grid), vec![11]);
  }

  #[test]
  fn test_all_zeros() {
    let grid = vec![vec![0, 0], vec![0, 0], vec![0, 0]];
    assert_eq!(Solution::find_column_width(grid), vec![1, 1]);
  }
}