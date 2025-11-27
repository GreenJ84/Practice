// A binary watch has 4 LEDs on the top to represent the hours (0-11), and 6 LEDs on the bottom to represent the minutes (0-59). Each LED represents a zero or one, with the least significant bit on the right.

// Given an integer turnedOn which represents the number of LEDs that are currently on (ignoring the PM), return all possible times the watch could represent. You may return the answer in any order.

// The hour must not contain a leading zero.
// - For example, "01:00" is not valid. It should be "1:00".
// The minute must consist of two digits and may contain a leading zero.
// - For example, "10:2" is not valid. It should be "10:02".


struct Solution;
impl Solution {
  pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    match turned_on {
      0 => {
        return vec!["0:00".to_string()];
      },
      1..=8 => {
        let mut result = Vec::new();

        fn count_ones(n: usize) -> usize{
          format!("{:b}", n).chars().filter(|ch| *ch == '1').count()
        }
        for hour in 0..12 {
          for min in 0..=59 {
            if count_ones(hour) + count_ones(min) == turned_on as usize {
              result.push(format!("{}:{:02}", hour, min));
            }
          }
        }

        return result;
      },
      _ => { return vec![]; }
    }
  }
}


#[cfg(test)]
mod tests {
  use super::Solution;

  fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v
  }

  #[test]
  fn test_turned_on_0() {
    let mut res = Solution::read_binary_watch(0);
    res.sort();
    assert_eq!(res, vec!["0:00".to_string()]);
  }

  #[test]
  fn test_turned_on_1_matches_example() {
    let expected = vec![
      "0:01","0:02","0:04","0:08","0:16","0:32",
      "1:00","2:00","4:00","8:00",
    ].into_iter().map(String::from).collect();
    assert_eq!(sorted(Solution::read_binary_watch(1)), sorted(expected));
  }

  #[test]
  fn test_turned_on_2_count() {
    // there are 44 valid times with exactly 2 LEDs on
    let res = Solution::read_binary_watch(2);
    assert_eq!(res.len(), 44);
  }

  #[test]
  fn test_turned_on_9_empty() {
    let res = Solution::read_binary_watch(9);
    assert!(res.is_empty());
  }

  #[test]
  fn test_turned_on_10_empty() {
    let res = Solution::read_binary_watch(10);
    assert!(res.is_empty());
  }
}