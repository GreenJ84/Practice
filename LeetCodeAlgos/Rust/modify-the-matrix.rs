// Given a 0-indexed m x n integer matrix matrix, create a new 0-indexed matrix called answer. Make answer equal to matrix, then replace each element with the value -1 with the maximum element in its respective column.

// Return the matrix answer.

struct Solution;
impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = matrix.clone();
        {
          let m = matrix.len();
          let n = matrix[0].len();

          let mut neg_indices = Vec::<usize>::new();
          let mut max = 0;
          for col in 0..n {
            for row in 0..m {
              let val = matrix[row][col];
              if val == -1 {
                neg_indices.push(row);
              } else if val > max {
                max = val;
              }
            }
            for idx in &neg_indices {
              ans[*idx][col] = max;
            }
            neg_indices.clear();
            max = 0;
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
    let matrix = vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]];
    let expected = vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]];
    assert_eq!(Solution::modified_matrix(matrix), expected);
  }

  #[test]
  fn test_example_2() {
    let matrix = vec![vec![3, -1], vec![5, 2]];
    let expected = vec![vec![3, 2], vec![5, 2]];
    assert_eq!(Solution::modified_matrix(matrix), expected);
  }

  #[test]
  fn test_single_negative_per_column() {
    let matrix = vec![vec![-1, 10], vec![5, -1]];
    let expected = vec![vec![5, 10], vec![5, 10]];
    assert_eq!(Solution::modified_matrix(matrix), expected);
  }

  #[test]
  fn test_all_negatives_except_one_each() {
    let matrix = vec![vec![-1, 1], vec![100, -1]];
    let expected = vec![vec![100, 1], vec![100, 1]];
    assert_eq!(Solution::modified_matrix(matrix), expected);
  }

  #[test]
  fn test_no_negatives() {
    let matrix = vec![vec![1, 2], vec![3, 4]];
    let expected = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(Solution::modified_matrix(matrix), expected);
  }

  #[test]
  fn test_larger_matrix() {
    let matrix = vec![
      vec![10, -1, 5],
      vec![-1, 20, -1],
      vec![15, 10, 25],
    ];
    let expected = vec![
      vec![10, 20, 5],
      vec![15, 20, 25],
      vec![15, 10, 25],
    ];
    assert_eq!(Solution::modified_matrix(matrix), expected);
  }

  #[test]
  fn test_minimum_dimensions() {
    let matrix = vec![vec![0, -1], vec![-1, 1]];
    let expected = vec![vec![0, 1], vec![0, 1]];
    assert_eq!(Solution::modified_matrix(matrix), expected);
  }
}