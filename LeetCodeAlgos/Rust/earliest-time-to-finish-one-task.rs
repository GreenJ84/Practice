// You are given a 2D integer array tasks where tasks[i] = [si, ti].

// Each [si, ti] in tasks represents a task with start time si that takes ti units of time to finish.

// Return the earliest time at which at least one task is finished.


struct Solution;
impl Solution {
  pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
    tasks.iter()
      .map(|t| t[0] + t[1])
      .min()
      .unwrap_or(0)
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn example1() {
    let tasks = vec![vec![1, 6], vec![2, 3]];
    assert_eq!(Solution::earliest_time(tasks), 5);
  }

  #[test]
  fn example2() {
    let tasks = vec![vec![100, 100], vec![100, 100], vec![100, 100]];
    assert_eq!(Solution::earliest_time(tasks), 200);
  }

  #[test]
  fn single_task_min_values() {
    let tasks = vec![vec![1, 1]];
    assert_eq!(Solution::earliest_time(tasks), 2);
  }

  #[test]
  fn multiple_varied_tasks() {
    let tasks = vec![vec![1, 2], vec![3, 1], vec![2, 2]];
    // finish times: 3, 4, 4 -> earliest 3
    assert_eq!(Solution::earliest_time(tasks), 3);
  }

  #[test]
  fn same_finish_times() {
    let tasks = vec![vec![5, 5], vec![2, 8], vec![4, 6]];
    // all finish at time 10
    assert_eq!(Solution::earliest_time(tasks), 10);
  }

  #[test]
  fn unsorted_input() {
    let tasks = vec![vec![10, 1], vec![1, 10], vec![5, 5]];
    // finish times: 11, 11, 10 -> earliest 10
    assert_eq!(Solution::earliest_time(tasks), 10);
  }

  #[test]
  fn multiple_with_same_start() {
    let tasks = vec![vec![1, 1], vec![1, 2], vec![1, 3]];
    // finish times: 2, 3, 4 -> earliest 2
    assert_eq!(Solution::earliest_time(tasks), 2);
  }
}
