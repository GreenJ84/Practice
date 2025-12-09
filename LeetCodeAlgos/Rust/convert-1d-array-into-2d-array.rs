// You are given a 0-indexed 1-dimensional (1D) integer array original, and two integers, m and n. You are tasked with creating a 2-dimensional (2D) array with  m rows and n columns using all the elements from original.

// The elements from indices 0 to n - 1 (inclusive) of original should form the first row of the constructed 2D array, the elements from indices n to 2 * n - 1 (inclusive) should form the second row of the constructed 2D array, and so on.

// Return an m x n 2D array constructed according to the above procedure, or an empty 2D array if it is impossible.

struct Solution;
impl Solution {
  pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    if original.len() != (m * n) as usize {
      return Vec::new();
    }
    original.chunks(n as usize)
      .map(|chunk| chunk.to_vec())
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2),
      vec![vec![1, 2], vec![3, 4]]
    );
  }

  #[test]
  fn test_example_2() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2, 3], 1, 3),
      vec![vec![1, 2, 3]]
    );
  }

  #[test]
  fn test_example_3() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2], 1, 1),
      Vec::<Vec<i32>>::new()
    );
  }

  #[test]
  fn test_single_element() {
    assert_eq!(
      Solution::construct2_d_array(vec![1], 1, 1),
      vec![vec![1]]
    );
  }

  #[test]
  fn test_single_row() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2, 3, 4, 5], 1, 5),
      vec![vec![1, 2, 3, 4, 5]]
    );
  }

  #[test]
  fn test_single_column() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2, 3, 4], 4, 1),
      vec![vec![1], vec![2], vec![3], vec![4]]
    );
  }

  #[test]
  fn test_impossible_too_many_rows() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2, 3], 2, 2),
      Vec::<Vec<i32>>::new()
    );
  }

  #[test]
  fn test_impossible_too_few_elements() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2, 3, 4, 5], 2, 3),
      Vec::<Vec<i32>>::new()
    );
  }

  #[test]
  fn test_larger_array() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3),
      vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    );
  }

  #[test]
  fn test_rectangular_array() {
    assert_eq!(
      Solution::construct2_d_array(vec![1, 2, 3, 4, 5, 6], 2, 3),
      vec![vec![1, 2, 3], vec![4, 5, 6]]
    );
  }
}
