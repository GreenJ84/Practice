// You are given a string of length 5 called time, representing the current time on a digital clock in the format "hh:mm". The earliest possible time is "00:00" and the latest possible time is "23:59".

// In the string time, the digits represented by the ? symbol are unknown, and must be replaced with a digit from 0 to 9.

// Return an integer answer, the number of valid clock times that can be created by replacing every ? with a digit from 0 to 9.

// Constraints:
// time is a valid string of length 5 in the format "hh:mm".
// "00" <= hh <= "23"
// "00" <= mm <= "59"
// Some of the digits might be replaced with '?' and need to be replaced with digits from 0 to 9.

struct Solution;
impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut ans = 1;
        let time = time.chars().collect::<Vec<char>>();
        for i in 0..5 {
            if i == 2 {
                continue;
            }
            match (i, time[i] == '?') {
                (0, true) => match time[1] {
                    '?' => {
                        ans *= 24;
                    }
                    x if x < '4' => {
                        ans *= 3;
                    }
                    _ => {
                        ans *= 2;
                    }
                },
                (1, true) => match time[0] {
                    '?' => {
                        continue;
                    }
                    x if x < '2' => {
                        ans *= 10;
                    }
                    _ => {
                        ans *= 4;
                    }
                },
                (3, true) => {
                    ans *= 6;
                }
                (4, true) => {
                    ans *= 10;
                }
                _ => {}
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let time = "?5:00".to_string();
        let result = Solution::count_time(time);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let time = "0?:0?".to_string();
        let result = Solution::count_time(time);
        assert_eq!(result, 100);
    }

    #[test]
    fn test_3() {
        let time = "??:??".to_string();
        let result = Solution::count_time(time);
        assert_eq!(result, 1440);
    }
}
