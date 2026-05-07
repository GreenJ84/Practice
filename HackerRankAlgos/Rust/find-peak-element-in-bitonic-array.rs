// Given a bitonic array (strictly increasing then strictly decreasing), find the index of the maximum element in O(log n) time.

// Constraints
// - 1 <= counts.length <= 100000
// - 0 <= counts[i] <= 1000000 for all 0 <= i < counts.length
// - All counts[i] are distinct
// - There exists an index p (0 < p < counts.length - 1) such that:
// - for all 1 <= i <= p: counts[i] > counts[i - 1]
// - for all p + 1 <= i < counts.length: counts[i] < counts[i - 1]

// fn findPeakIndex(counts: &[i32]) -> i32 {
fn find_peak_index(counts: &[i32]) -> i32 {
  let (mut left, mut right) = (0usize, counts.len());
  while left < right {
    let mid = (left + right) / 2;
    if counts[mid - 1] < counts[mid] && counts[mid] > counts[mid + 1] {
      return mid as i32;
    } else if counts[mid - 1] > counts[mid] {
      right = mid;
    } else {
      left = mid;
    }
  }
  -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let counts = [1, 3, 5, 4, 2];
        assert_eq!(find_peak_index(&counts), 2);
    }

    #[test]
    fn test_2() {
        let counts = [11, 21, 31, 41, 51, 61, 71, 81, 91, 100, 99, 89, 79, 69, 59, 49, 39, 29, 19];
        assert_eq!(find_peak_index(&counts), 9);
    }

    #[test]
    fn test_3() {
        let counts = [
            10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 91, 81, 71, 61, 51, 41, 31, 21, 11,
        ];
        assert_eq!(find_peak_index(&counts), 9);
    }
}

// Example

// Input:

// counts = [1, 3, 5, 4, 2]
// Output:

// 2
// Explanation:

// We perform a binary search on counts:

// low = 0, high = 4. mid = (0 + 4) // 2 = 2. counts[2] = 5, counts[3] = 4. Since 5 > 4, we are on the descending side, so we move high = mid = 2.
// low = 0, high = 2. mid = (0 + 2) // 2 = 1. counts[1] = 3, counts[2] = 5. Since 3 < 5, we are on the ascending side, so we move low = mid + 1 = 2.
// Now low = 2, high = 2, loop ends and we return 2, which is the index of the maximum element (5).
