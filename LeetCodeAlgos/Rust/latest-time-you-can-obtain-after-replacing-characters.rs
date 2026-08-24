// You are given a string s representing a 12-hour format time where some of the digits (possibly none) are replaced with a "?".

// 12-hour times are formatted as "HH:MM", where HH is between 00 and 11, and MM is between 00 and 59. The earliest 12-hour time is 00:00, and the latest is 11:59.

// You have to replace all the "?" characters in s with digits such that the time we obtain by the resulting string is a valid 12-hour format time and is the latest possible.

// Return the resulting string.

// Constraints:
// s.length == 5
// s[2] is equal to the character ":".
// All characters except s[2] are digits or "?" characters.
// The input is generated such that there is at least one time between "00:00" and "11:59" that you can obtain after replacing the "?" characters.

struct Solution;
impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            match (i, chars[i] == '?') {
                (2, _) | (_, false) => {}
                (0, true) => {
                    chars[i] = match chars[1] {
                        v if v < '2' || v == '?' => '1',
                        _ => '0',
                    };
                }
                (1, true) => {
                    chars[i] = match chars[0] {
                        '1' => '1',
                        _ => '9',
                    };
                }
                (3, true) => {
                    chars[3] = '5';
                }
                (4, true) => {
                    chars[4] = '9';
                }
                _ => {}
            }
        }
        chars.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "1?:?4".to_string();
        let result = Solution::find_latest_time(s);
        assert_eq!(result, "11:54");
    }

    #[test]
    fn test_2() {
        let s = "0?:5?".to_string();
        let result = Solution::find_latest_time(s);
        assert_eq!(result, "09:59");
    }
}
