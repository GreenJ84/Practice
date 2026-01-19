// You are given a m x n matrix grid consisting of non-negative integers.

// In one operation, you can increment the value of any grid[i][j] by 1.

// Return the minimum number of operations needed to make all columns of grid strictly increasing.

struct Solution;
impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut ops = 0;

        let mut last_row = grid[0].clone();
        for row in 1..grid.len() {
          for col in 0..last_row.len() {
            match (last_row[col], grid[row][col]) {
              (last, curr) if curr > last => {
                last_row[col] = curr;
                continue;
              },
              (last, curr) if curr < last => {
                ops += last - curr + 1;
              },
              (_, _) => {
                ops += 1;
              }
            }
            last_row[col] += 1;
          }
        }
        ops
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let grid = vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]];
    assert_eq!(Solution::minimum_operations(grid), 15);
  }

  #[test]
  fn test_example_2() {
    let grid = vec![vec![3, 2, 1], vec![2, 1, 0], vec![1, 2, 3]];
    assert_eq!(Solution::minimum_operations(grid), 12);
  }

  #[test]
  fn test_single_row() {
    let grid = vec![vec![1, 2, 3]];
    assert_eq!(Solution::minimum_operations(grid), 0);
  }

  #[test]
  fn test_single_column() {
    let grid = vec![vec![1], vec![2], vec![3]];
    assert_eq!(Solution::minimum_operations(grid), 0);
  }

  #[test]
  fn test_already_strictly_increasing() {
    let grid = vec![vec![1, 5], vec![2, 6], vec![3, 7]];
    assert_eq!(Solution::minimum_operations(grid), 0);
  }

  #[test]
  fn test_all_zeros() {
    let grid = vec![vec![0, 0], vec![0, 0]];
    assert_eq!(Solution::minimum_operations(grid), 2);
  }

  #[test]
  fn test_single_cell() {
    let grid = vec![vec![5]];
    assert_eq!(Solution::minimum_operations(grid), 0);
  }

  #[test]
  fn test_large_values() {
    let grid = vec![vec![2400, 2400], vec![2400, 2400]];
    assert_eq!(Solution::minimum_operations(grid), 2);
  }
}