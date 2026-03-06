// You are given a string date representing a Gregorian calendar date in the yyyy-mm-dd format.

// date can be written in its binary representation obtained by converting year, month, and day to their binary representations without any leading zeroes and writing them down in year-month-day format.

// Return the binary representation of date.

struct Solution;
impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date
          .split('-')
          .map(|part| {
            let val = part.parse::<i32>().unwrap();
            format!("{val:b}")
          })
          .collect::<Vec<String>>()
          .join("-")
    }

    pub fn c_onvert_date_to_binary2(date: String) -> String {
        let parts: Vec<&str> = date.split('-').collect();
        let year = parts[0].parse::<i32>().unwrap();
        let month = parts[1].parse::<i32>().unwrap();
        let day = parts[2].parse::<i32>().unwrap();

        format!("{year:b}-{month:b}-{day:b}")
    }

    pub fn _convert_date_to_binary1(date: String) -> String {
        let parts: Vec<&str> = date.split('-').collect();
        let year = parts[0].parse::<i32>().unwrap();
        let month = parts[1].parse::<i32>().unwrap();
        let day = parts[2].parse::<i32>().unwrap();

        format!(
            "{}-{}-{}",
            format!("{:b}", year),
            format!("{:b}", month),
            format!("{:b}", day)
        )
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
      let date = "2080-02-29".to_string();
      let result = "100000100000-10-11101".to_string();
      assert_eq!(Solution::convert_date_to_binary(date), result);
  }

  #[test]
  fn test_2() {
      let date = "1900-01-01".to_string();
      let result = "11101101100-1-1".to_string();
      assert_eq!(Solution::convert_date_to_binary(date), result);
  }
}