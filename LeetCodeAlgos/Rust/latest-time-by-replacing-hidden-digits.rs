// You are given a string time in the form of  hh:mm, where some of the digits in the string are hidden (represented by ?).

// The valid times are those inclusively between 00:00 and 23:59.

// Return the latest valid time you can get from time by replacing the hidden digits.

struct Solution;
impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut max_time: Vec<char> = time.chars().collect();
        for idx in 0..5 {
            if idx == 2 || max_time[idx] != '?' {
                continue;
            }
            max_time[idx] = match idx {
                0 => {
                    if max_time[1] == '?' || max_time[1] as u8 - b'0' < 4 {
                        '2'
                    } else {
                        '1'
                    }
                }
                1 => {
                    if max_time[0] == '2' {
                        '3'
                    } else {
                        '9'
                    }
                }
                3 => '5',
                _ => '9',
            };
        }
        max_time.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::maximum_time("2?:?0".to_string()), "23:50");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::maximum_time("0?:3?".to_string()), "09:39");
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::maximum_time("1?:22".to_string()), "19:22");
    }

    #[test]
    fn test_no_hidden_digits() {
        assert_eq!(Solution::maximum_time("23:59".to_string()), "23:59");
    }

    #[test]
    fn test_all_hidden_digits() {
        assert_eq!(Solution::maximum_time("?:??".to_string()), "23:59");
    }

    #[test]
    fn test_hidden_hour_first_digit() {
        assert_eq!(Solution::maximum_time("?3:45".to_string()), "23:45");
    }

    #[test]
    fn test_hidden_hour_second_digit() {
        assert_eq!(Solution::maximum_time("1?:00".to_string()), "19:00");
    }

    #[test]
    fn test_hidden_minute_first_digit() {
        assert_eq!(Solution::maximum_time("12:?5".to_string()), "12:55");
    }

    #[test]
    fn test_hidden_minute_second_digit() {
        assert_eq!(Solution::maximum_time("00:0?".to_string()), "00:09");
    }

    #[test]
    fn test_midnight() {
        assert_eq!(Solution::maximum_time("0?:0?".to_string()), "09:09");
    }
}
