// Given a sorted array of unique integers that has been rotated at an unknown pivot, find the index of a target value or return -1 if not found.

// Constraints
// - 0 <= nums.length <= 100000
// - 0 <= nums[i] <= 10^18
// - All elements in nums are unique
// - nums is obtained by taking a strictly increasing sorted array and rotating it at an unknown pivot
// - 0 <= target <= 10^18

// fn searchRotatedTimestamps(nums: &[i32], target: i32) -> i32 {
fn search_rotated_timestamps(nums: &[i32], target: i32) -> i32 {
  if nums.is_empty() {
    return -1;
  }
  let mut left = 0;
  if nums[left] == target {
    return left as i32;
  }
  let mut right = nums.len() - 1;
  if nums[right] == target {
    return right as i32;
  }

  while left < right {
    let mid = left + (right - left) / 2;
    if nums[mid] == target {
      return mid as i32;
    } else if left == mid || right == mid {
      break;
    }

    if nums[left] < nums[mid] {
      if nums[left] < target && target < nums[mid] {
        right = mid;
      }
      else {
        left = mid;
      }
    }
    else if nums[mid] < nums[right] {
      if nums[mid] < target && target < nums[right] {
        left = mid;
      } else {
        right = mid;
      }
    }
  }
  -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [1609466400, 1609470000, 1609473600, 1609459200, 1609462800];
        let target = 1609459200;
        assert_eq!(search_rotated_timestamps(&nums, target), 3);
    }

    #[test]
    fn test_2() {
        let nums = [1609466400, 1609470000, 1609473600, 1609459200, 1609462800];
        let target = 1609462800;
        assert_eq!(search_rotated_timestamps(&nums, target), 4);
    }

    #[test]
    fn test_3() {
        let nums = [1609466400, 1609470001, 1609473600, 1609459200, 1609462800];
        let target = 1609470000;
        assert_eq!(search_rotated_timestamps(&nums, target), -1);
    }
}
