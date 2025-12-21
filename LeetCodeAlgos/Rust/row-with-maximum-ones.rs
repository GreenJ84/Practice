// Given a m x n binary matrix mat, find the 0-indexed position of the row that contains the maximum count of ones, and the number of ones in that row.

// In case there are multiple rows that have the maximum count of ones, the row with the smallest row number should be selected.

// Return an array containing the index of the row, and the number of ones in it.

struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
      let ans = mat.iter().enumerate()
        .map(|(idx, row)|
          (idx as i32, row.iter().filter(|&cell| cell == &1).count() as i32)
        )
        .inspect(|x| println!("After map: {:?}", x))
        .max_by(|x, y|
          match x.1.cmp(&y.1) {
            Ordering::Equal => y.0.cmp(&x.0),
            x => x
          }
        )
        .inspect(|x| println!("After max_by: {:?}", x))
        .unwrap();

      vec![ans.0, ans.1]
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let mat = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![0, 1]);
  }

  #[test]
  fn example_2() {
    let mat = vec![vec![0, 0, 0], vec![0, 1, 1]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![1, 2]);
  }

  #[test]
  fn example_3() {
    let mat = vec![vec![0, 0], vec![1, 1], vec![0, 0]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![1, 2]);
  }

  #[test]
  fn single_row_all_ones() {
    let mat = vec![vec![1, 1, 1, 1]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![0, 4]);
  }

  #[test]
  fn single_row_all_zeros() {
    let mat = vec![vec![0, 0, 0]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![0, 0]);
  }

  #[test]
  fn all_zeros() {
    let mat = vec![vec![0, 0], vec![0, 0], vec![0, 0]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![0, 0]);
  }

  #[test]
  fn all_ones() {
    let mat = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![0, 2]);
  }

  #[test]
  fn multiple_max_rows_smallest_index() {
    let mat = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![0, 2]);
  }

  #[test]
  fn single_column() {
    let mat = vec![vec![0], vec![1], vec![0]];
    assert_eq!(Solution::row_and_maximum_ones(mat), vec![1, 1]);
  }
}