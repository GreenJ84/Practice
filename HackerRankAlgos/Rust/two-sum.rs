// Given an array of positive integers and a target integer, return the indices of two elements that sum to the target or [-1, -1] if no such pair exists.

// fn findTaskPairForSlot(taskDurations: &[i32], slotLength: i32) -> Vec<i32> {
fn find_task_pair_for_slot(task_durations: &[i32], slot_length: i32) -> Vec<i32> {
  let mut pair = std::collections::HashMap::<i32, i32>::with_capacity(1000);
  for idx in 0..task_durations.len() {
    if task_durations[idx] >= slot_length {
      continue;
    }

    if let Some(&first_idx) = pair.get(&task_durations[idx]) {
      return vec![first_idx, idx as i32];
    } else {
      pair.insert(slot_length - task_durations[idx], idx as i32);
    }
  }
  vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let task_durations = vec![2, 7, 11, 15];
        let slot_length = 9;
        assert_eq!(find_task_pair_for_slot(&task_durations, slot_length), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let task_durations = vec![1, 2, 3, 4];
        let slot_length = 8;
        assert_eq!(find_task_pair_for_slot(&task_durations, slot_length), vec![-1, -1]);
    }

    #[test]
    fn test_3() {
        let task_durations = vec![1, 2, 3, 4];
        let slot_length = 7;
        assert_eq!(find_task_pair_for_slot(&task_durations, slot_length), vec![2, 3]);
    }
}

// Example 1

// Input:

// task_durations = [2, 7, 11, 15]
// slot_length = 9
// Output:

// [0, 1]
// Explanation:

// We look for two durations that sum to the slot_length. Starting with index 0 (2), we need 7 (9-2). At index 1, duration is 7. Thus indices [0, 1] sum to 9.

// Example 2

// Input:

// task_durations = [1, 2, 3, 4]
// slot_length = 8
// Output:

// [-1, -1]
// Explanation:

// No two durations in the list sum to 8. All pairs are: 1+2=3, 1+3=4, 1+4=5, 2+3=5, 2+4=6, 3+4=7. None match 8.