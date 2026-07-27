// You are given two valid times startTime and endTime, each represented as a string in the format "HH:MM:SS".

// Return the number of seconds that have elapsed from startTime to endTime.

// Constraints:
// startTime.length == 8
// endTime.length == 8
// startTime and endTime are valid times in the format "HH:MM:SS"
// 00 <= HH <= 23

struct Solution;
impl Solution {
    pub fn seconds_between_times(start_time: String, end_time: String) -> i32 {
        let start_parts: Vec<i32> = start_time
            .split(':')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut end_parts: Vec<i32> = end_time
            .split(':')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut ans = 0i32;
        for i in (0..=2).rev() {
            if start_parts[i] > end_parts[i] {
                match i {
                    2 | 1 => {
                        end_parts[i] += 60;
                        end_parts[i - 1] -= 1;
                    }
                    _ => {
                        panic!("End time comes before Start time!!")
                    }
                }
            }
        }
        ans += end_parts[2] - start_parts[2];
        ans += (end_parts[1] - start_parts[1]) * 60;
        ans + (end_parts[0] - start_parts[0]) * 3600
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let start_time = "01:00:00".to_string();
        let end_time = "01:00:25".to_string();
        let result = Solution::seconds_between_times(start_time, end_time);
        assert_eq!(result, 25);
    }

    #[test]
    fn test_2() {
        let start_time = "12:34:56".to_string();
        let end_time = "13:00:00".to_string();
        let result = Solution::seconds_between_times(start_time, end_time);
        assert_eq!(result, 1504);
    }
}
