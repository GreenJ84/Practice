// Given a non-decreasing array of integers and an integer K, remove in-place any element that is within K of the previous kept element and return the new length. Use constant extra space and single pass with two pointers.

// fn debounceTimestamps(timestamps: &[i32], K: i32) -> i32 {
fn debounce_timestamps(timestamps: &[i32], k: i32) -> i32 {
  if timestamps.is_empty() {
    return 0;
  }
  let (mut ans, mut left) = (1, 0);
  for right in 1..timestamps.len() {
    if timestamps[right] - timestamps[left] >= k {
      left = right;
      ans += 1;
    }
  }
  ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let timestamps = vec![1, 2, 3, 8, 10];
        let k = 3;
        assert_eq!(debounce_timestamps(&timestamps, k), 2);
    }

    #[test]
    fn test_2() {
        let timestamps = vec![1, 2, 3, 4, 5];
        let k = 1;
        assert_eq!(debounce_timestamps(&timestamps, k), 5);
    }

    #[test]
    fn test_3() {
        let timestamps = vec![1, 2, 3, 4, 5];
        let k = 2;
        assert_eq!(debounce_timestamps(&timestamps, k), 3);
    }
}

// Example

// Input:

// timestamps = [1, 2, 3, 8, 10]
// K = 3
// Output:

// 2
// Explanation:

// We start by keeping the first timestamp 1.
// Next, 2 − 1 = 1 < 3, so 2 is removed.
// Next, 3 − 1 = 2 < 3, so 3 is removed.
// Next, 8 − 1 = 7 ≥ 3, so we keep 8.
// Finally, 10 − 8 = 2 < 3, so 10 is removed.
// The remaining timestamps are [1, 8], so the new length is 2.