// You are given a 1-indexed array of distinct integers nums of length n.

// You need to distribute all the elements of nums between two arrays arr1 and arr2 using n operations. In the first operation, append nums[1] to arr1. In the second operation, append nums[2] to arr2. Afterwards, in the ith operation:

// If the last element of arr1 is greater than the last element of arr2, append nums[i] to arr1. Otherwise, append nums[i] to arr2.
// The array result is formed by concatenating the arrays arr1 and arr2. For example, if arr1 == [1,2,3] and arr2 == [4,5,6], then result = [1,2,3,4,5,6].

// Return the array result.

// Constraints:
// 3 <= n <= 50
// 1 <= nums[i] <= 100
// All elements in nums are distinct.

struct Solution;
impl Solution {
    pub fn result_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut j = 1usize;
        let mut odd = vec![nums[j]];
        for i in 2..n {
          if &nums[j - 1] > odd.last().unwrap() {
            nums[j] = nums[i];
            j += 1;
          } else {
            odd.push(nums[i]);
          }
        }
        for num in odd {
          nums[j] = num;
          j += 1;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
      let nums = vec![2, 1, 3];
        assert_eq!(Solution::result_array(nums), vec![2, 3, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![5, 4, 3, 8];
        assert_eq!(Solution::result_array(nums), vec![5, 3, 4, 8]);
    }
}