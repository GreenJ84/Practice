// Given an array of integers arr, replace each element with its rank.

// The rank represents how large the element is. The rank has the following rules:
// - Rank is an integer starting from 1.
// - The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
// - Rank should be as small as possible.

struct Solution;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
      let mut indices: Vec<_> = (0..arr.len()).collect();
      indices.sort_unstable_by_key(|&v| arr[v]);

      let mut rank = 0;
      let mut last = None;

      for i in indices {
          if last != Some(arr[i]) {
              last = Some(arr[i]);
              rank += 1;
          }
          arr[i] = rank;
      }

      arr
    }

    pub fn array_rank_transform1(arr: Vec<i32>) -> Vec<i32> {
        let mut ans = arr.clone();
        let mut map = std::collections::HashMap::<&i32, Vec<usize>>::new();

        for idx in 0..arr.len() {
          map.entry(&arr[idx])
            .and_modify(|v| v.push(idx))
            .or_insert(vec![idx]);
        }
        let mut keys: Vec<&&i32> = map.keys().collect();
        keys.sort();

        let mut rank = 1;
        for key in keys {
          for idx in map.get(key).unwrap() {
            ans[*idx] = rank;
          }
          rank += 1;
        }
        ans
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(Solution::array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]), vec![5, 3, 4, 2, 8, 6, 7, 1, 3]);
  }

  #[test]
  fn empty_array() {
    assert_eq!(Solution::array_rank_transform(vec![]), vec![]);
  }

  #[test]
  fn single_element() {
    assert_eq!(Solution::array_rank_transform(vec![42]), vec![1]);
  }

  #[test]
  fn negative_numbers() {
    assert_eq!(Solution::array_rank_transform(vec![-5, -1, -3]), vec![1, 3, 2]);
  }

  #[test]
  fn mixed_positive_negative() {
    assert_eq!(Solution::array_rank_transform(vec![-10, 0, 10, 5]), vec![1, 2, 4, 3]);
  }

  #[test]
  fn duplicates_with_others() {
    assert_eq!(Solution::array_rank_transform(vec![5, 5, 10, 2]), vec![2, 2, 3, 1]);
  }

  #[test]
  fn ascending_order() {
    assert_eq!(Solution::array_rank_transform(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn descending_order() {
    assert_eq!(Solution::array_rank_transform(vec![5, 4, 3, 2, 1]), vec![5, 4, 3, 2, 1]);
  }

  #[test]
  fn large_range() {
    assert_eq!(Solution::array_rank_transform(vec![-1000000000, 0, 1000000000]), vec![1, 2, 3]);
  }
}