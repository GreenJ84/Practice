// Given two integer arrays startTime and endTime and given an integer queryTime.

// The ith student started doing their homework at the time startTime[i] and finished it at time endTime[i].

// Return the number of students doing their homework at time queryTime. More formally, return the number of students where queryTime lays in the interval [startTime[i], endTime[i]] inclusive.

struct Solution;
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        (0usize..start_time.len()).filter(|&i|
          query_time >= start_time[i] && query_time <= end_time[i]
        ).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
      let start_time = vec![1,2,3];
      let end_time = vec![3,2,7];
      let query_time = 4;
      let result = Solution::busy_student(start_time, end_time, query_time);
      assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
      let start_time = vec![4];
      let end_time = vec![4];
      let query_time = 4;
      let result = Solution::busy_student(start_time, end_time, query_time);
      assert_eq!(result, 1);
    }
}
