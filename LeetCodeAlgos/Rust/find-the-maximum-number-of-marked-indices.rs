// You are given a 0-indexed integer array nums.

// Initially, all of the indices are unmarked. You are allowed to make this operation any number of times:

// Pick two different unmarked indices i and j such that 2 * nums[i] <= nums[j], then mark i and j.
// Return the maximum possible number of marked indices in nums using the above operation any number of times.

// Constraints:
// - 1 <= nums.length <= 105
// - 1 <= nums[i] <= 109

struct Solution;
impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
      nums.sort_unstable();
      println!("{}\n{nums:?}", nums.len());
      let mut right = nums.len() - 1;
      let mut left = nums.len() / 2 - 1;
      let mut ans = 0;
      for _ in 0..nums.len() / 2 {
        if nums[left] * 2 <= nums[right]{
          nums[left] = -1;
          nums[right] = 1000000001;
          right -= 1;
          ans += 2;
        }
        if left == 0 {
          break;
        }
        left -= 1;
      }
      ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 5, 2, 4];
        let res = Solution::max_num_of_marked_indices(nums);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![9, 2, 5, 4];
        let res = Solution::max_num_of_marked_indices(nums);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_3() {
        let nums = vec![7, 6, 8];
        let res = Solution::max_num_of_marked_indices(nums);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_4() {
      let nums = vec![57,40,57,51,90,51,68,100,24,39,11,85,2,22,67,29,74,82,10,96,14,35,25,76,26,54,29,44,63,49,73,50,95,89,43,62,24,88,88,36,6,16,14,2,42,42,60,25,4,58,23,22,27,26,3,79,64,20,92];
      let res = Solution::max_num_of_marked_indices(nums);
      assert_eq!(res, 58);
    }
}
