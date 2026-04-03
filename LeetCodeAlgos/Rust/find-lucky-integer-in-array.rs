// Given an array of integers arr, a lucky integer is an integer that has a frequency in the array equal to its value.

// Return the largest lucky integer in the array. If there is no lucky integer return -1.

struct Solution;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = std::collections::HashMap::<i32, i32>::new();
        for num in arr {
          *freq.entry(num).or_insert(0) += 1;
        }

        let mut ans = -1i32;
        for (key, val) in freq {
          if key == val && val > ans {
            ans = val
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
        let arr = vec![2, 2, 3, 4];
        assert_eq!(Solution::find_lucky(arr), 2);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(Solution::find_lucky(arr), 3);
    }

    #[test]
    fn test_3() {
        let arr = vec![2, 2, 2, 3, 3];
        assert_eq!(Solution::find_lucky(arr), -1);
    }
}
