// Given an array of intervals [startTime, endTime], merge all overlapping intervals and return a sorted array of non-overlapping intervals.


// Return a 2D array of two space-separated integers start and end, representing the merged intervals sorted by increasing start time.

// fn mergeHighDefinitionIntervals(intervals: &[Vec<i32>]) -> Vec<Vec<i32>> {
fn merge_high_definition_intervals(intervals: &[Vec<i32>]) -> Vec<Vec<i32>> {
  let mut intervals = intervals.to_vec();
  intervals.sort_unstable_by_key(|i| i[0]);

  let mut idx = 1;
  while idx < intervals.len() {
    if intervals[idx][0] <= intervals[idx - 1][1] {
      intervals[idx - 1][1] = intervals[idx - 1][1].max(intervals[idx][1]);
      intervals.remove(idx);
      idx -= 1;
    }
    idx += 1;
  }
  intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(merge_high_definition_intervals(&intervals), expected);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        assert_eq!(merge_high_definition_intervals(&intervals), expected);
    }
  }

// Constraints

// 0 <= intervals.length <= 100000
// intervals[i].length == 2 for all 0 <= i < intervals.length
// 0 <= intervals[i][0] < intervals[i][1] <= 10^9 for all 0 <= i < intervals.length