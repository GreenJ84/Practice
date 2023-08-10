// There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).

// Before being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].

// Given the array nums after the rotation and an integer target, return true if target is in nums, or false if it is not in nums.

// You must decrease the overall operation steps as much as possible.
struct Solution {}
impl Solution {
    pub fn search(mut nums: Vec<i32>, target: i32) -> bool {
        nums.dedup();
        let mut left = 0;
        let mut right = nums.len() - 1;
        if nums[left] == target || nums[right] == target {
            return true;
        }
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return true;
            } else if mid == 0{
                return false;
            }else if nums[mid] <= nums[right] {
                if nums[mid] <= target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                if nums[mid] >= target && target >= nums[left]{
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            println!("left: {}, right: {}", left, right);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 0), true);
    }

    #[test]
    fn test_search2() {
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 3), false);
    }

    #[test]
    fn test_search3() {
        assert_eq!(Solution::search(vec![1,0,1,1,1], 0), true);
    }
}
