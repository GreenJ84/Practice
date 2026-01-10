// Given an array of positive integers nums, return an array answer that consists of the digits of each integer in nums after separating them in the same order they appear in nums.

// To separate the digits of an integer is to get all the digits it has in the same order.

// For example, for the integer 10921, the separation of its digits is [1,0,9,2,1].

struct Solution;
impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
          .map(|num| Self::get_digits(num))
          .flatten()
          .collect()
    }

    fn get_digits(num: &i32) -> Vec<i32> {
        let mut arr = vec![];
        let mut n = *num;
        while n > 0 {
            arr.push(n % 10);
            n /= 10;
        }
        arr.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::separate_digits(vec![7, 1, 3, 9]),
            vec![7, 1, 3, 9]
        );
    }

    #[test]
    fn single_digit() {
        assert_eq!(Solution::separate_digits(vec![5]), vec![5]);
    }

    #[test]
    fn large_numbers() {
        assert_eq!(
            Solution::separate_digits(vec![100, 99999]),
            vec![1, 0, 0, 9, 9, 9, 9, 9]
        );
    }

    #[test]
    fn multiple_same_digits() {
        assert_eq!(
            Solution::separate_digits(vec![11, 22, 33]),
            vec![1, 1, 2, 2, 3, 3]
        );
    }

    #[test]
    fn max_constraint() {
        assert_eq!(
            Solution::separate_digits(vec![100000]),
            vec![1, 0, 0, 0, 0, 0]
        );
    }
}
