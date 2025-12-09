// You are given a 2D matrix grid of size m x n. You need to check if each cell grid[i][j] is:
// -  Equal to the cell below it, i.e. grid[i][j] == grid[i + 1][j] (if it exists).
// -  Different from the cell to its right, i.e. grid[i][j] != grid[i][j + 1] (if it exists).

// Return true if all the cells satisfy these conditions, otherwise, return false.

struct Solution;
impl Solution {
  pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
    for col in (0..grid[0].len() - 1).rev() {
      if grid[0][col] == grid[0][col + 1] {
        return false;
      }
    }
    for row in 1..grid.len() {
      for col in 0..grid[0].len() {
        if grid[row][col] != grid[row - 1][col] {
          return false;
        }
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let grid = vec![vec![1, 0, 2], vec![1, 0, 2]];
    assert_eq!(Solution::satisfies_conditions(grid), true);
  }

  #[test]
  fn test_example_2() {
    let grid = vec![vec![1, 1, 1], vec![0, 0, 0]];
    assert_eq!(Solution::satisfies_conditions(grid), false);
  }

  #[test]
  fn test_example_3() {
    let grid = vec![vec![1], vec![2], vec![3]];
    assert_eq!(Solution::satisfies_conditions(grid), false);
  }

  #[test]
  fn test_single_cell() {
    let grid = vec![vec![5]];
    assert_eq!(Solution::satisfies_conditions(grid), true);
  }

  #[test]
  fn test_single_row() {
    let grid = vec![vec![1, 2, 3]];
    assert_eq!(Solution::satisfies_conditions(grid), true);
  }

  #[test]
  fn test_all_same_values() {
    let grid = vec![vec![7, 7, 7], vec![7, 7, 7]];
    assert_eq!(Solution::satisfies_conditions(grid), false);
  }

  #[test]
  fn test_valid_columns_different_rows() {
    let grid = vec![vec![2, 3], vec![2, 4], vec![2, 5]];
    assert_eq!(Solution::satisfies_conditions(grid), false);
  }

  #[test]
  fn test_max_constraints() {
    let grid = vec![vec![9, 8, 7], vec![9, 8, 6]];
    assert_eq!(Solution::satisfies_conditions(grid), false);
  }
}
