// You are given two strings current and correct representing two 24-hour times.

// 24-hour times are formatted as "HH:MM", where HH is between 00 and 23, and MM is between 00 and 59. The earliest 24-hour time is 00:00, and the latest is 23:59.

// In one operation you can increase the time current by 1, 5, 15, or 60 minutes. You can perform this operation any number of times.

// Return the minimum number of operations needed to convert current to correct.

struct Solution;
impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
      let minutes = |s: String| -> i32 {
        let vals = s.split(":").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        vals[0] * 60 + vals[1]
      };

      let mut diff = minutes(correct) - minutes(current);
      let mut operations = 0;
      for num in [60, 15, 5] {
        operations += diff / num;
        diff %= num;
      }
      operations + diff
    }

    fn _parse_time(time: String) -> (i32, i32){
      let time = time.split(":").collect::<Vec<_>>();
      (time[0].parse::<i32>().unwrap(), time[1].parse::<i32>().unwrap())
    }

    pub fn _convert_time1(current: String, correct: String) -> i32 {
      let (cu_hour, cu_min) = Self::_parse_time(current);
      let (ex_hour, ex_min) = Self::_parse_time(correct);

      let mut operations = 0;
      operations += ex_hour - cu_hour;
      let mut diff = if cu_min > ex_min {
        operations -= 1;
        60 - cu_min + ex_min
      } else {
        ex_min - cu_min
      };
      for num in [15, 5] {
        operations += diff / num;
        diff %= num;
      }
      operations + diff
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let current = "02:30".to_string();
        let correct = "04:35".to_string();
        assert_eq!(Solution::convert_time(current, correct), 3);
    }

    #[test]
    fn test_2() {
        let current = "11:00".to_string();
        let correct = "11:01".to_string();
        assert_eq!(Solution::convert_time(current, correct), 1);
    }
}

// Example 1:

// Input: current = "02:30", correct = "04:35"
// Output: 3
// Explanation:
// We can convert current to correct in 3 operations as follows:
// - Add 60 minutes to current. current becomes "03:30".
// - Add 60 minutes to current. current becomes "04:30".
// - Add 5 minutes to current. current becomes "04:35".
// It can be proven that it is not possible to convert current to correct in fewer than 3 operations.
// Example 2:

// Input: current = "11:00", correct = "11:01"
// Output: 1
// Explanation: We only have to add one minute to current, so the minimum number of operations needed is 1.