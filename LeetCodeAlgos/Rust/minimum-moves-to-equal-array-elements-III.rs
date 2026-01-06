// You are given an integer array nums.

// In one move, you may increase the value of any single element nums[i] by 1.

// Return the minimum total number of moves required so that all elements in nums become equal.

struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
      if nums.len() == 1 { return 0; }

      let max = nums.iter().max().unwrap();
      nums.iter().fold(0, |acc, &num| acc + max - num )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_moves(vec![2, 1, 3]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_moves(vec![4, 4, 5]), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_moves(vec![5]), 0);
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(Solution::min_moves(vec![3, 3, 3]), 0);
    }

    #[test]
    fn test_ascending_order() {
        assert_eq!(Solution::min_moves(vec![1, 2, 3, 4]), 6);
    }

    #[test]
    fn test_descending_order() {
        assert_eq!(Solution::min_moves(vec![5, 4, 3, 2, 1]), 10);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::min_moves(vec![1, 100]), 99);
    }

    #[test]
    fn test_min_constraint() {
        assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
    }

    #[test]
    fn test_max_constraint() {
        assert_eq!(Solution::min_moves(vec![100, 100]), 0);
    }
}
