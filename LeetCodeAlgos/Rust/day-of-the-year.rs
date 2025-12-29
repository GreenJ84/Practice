// Given a string date representing a Gregorian calendar date formatted as YYYY-MM-DD, return the day number of the year.

struct Solution;
impl Solution {
  pub fn day_of_year(date: String) -> i32 {
    let [year, month, day, ..] = date.split("-").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()[..] else {
      panic!("Invalid date format");
    };
    let mut doy = day;

    // Leap year check
    // If after Feb, leap year check needed
    if month > 2 &&
      // every 4 (divisible) years
      year % 4 == 0 &&
      // not 100 year divisible (unless also 400 yr divisible)
      (year % 100 != 0 || year % 400 == 0)
    {
      doy += 1;
    }

    for m in 1..month {
      doy += match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28,
        _ => 0,
      }
    }
    doy
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
  }

  #[test]
  fn test_example_2() {

    assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
  }

  #[test]
  fn test_jan_01() {
    assert_eq!(Solution::day_of_year("2019-01-01".to_string()), 1);
  }

  #[test]
  fn test_dec_31() {
    assert_eq!(Solution::day_of_year("2019-12-31".to_string()), 365);
  }

  #[test]
  fn test_leap_year_dec_31() {
    assert_eq!(Solution::day_of_year("2020-12-31".to_string()), 366);
  }

  #[test]
  fn test_leap_year_feb_29() {
    assert_eq!(Solution::day_of_year("2020-02-29".to_string()), 60);
  }

  #[test]
  fn test_mar_01_non_leap() {
    assert_eq!(Solution::day_of_year("2019-03-01".to_string()), 60);
  }

  #[test]
  fn test_mar_01_leap() {
    assert_eq!(Solution::day_of_year("2020-03-01".to_string()), 61);
  }

  #[test]
  fn test_year_1900() {
    assert_eq!(Solution::day_of_year("1900-12-31".to_string()), 365);
  }
}
