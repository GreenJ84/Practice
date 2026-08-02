// You are given a list of songs where the ith song has a duration of time[i] seconds.

// Return the number of pairs of songs for which their total duration in seconds is divisible by 60. Formally, we want the number of indices i, j such that i < j with (time[i] + time[j]) % 60 == 0.

// Constraints:
// 1 <= time.length <= 6 * 10^4
// 1 <= time[i] <= 500

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        for &t in &time {
            let at = t % 60;
            if let Some(&comp) = map.get(&((60 - at) % 60)) {
                ans += comp;
            }
            *map.entry(at).or_insert(0) += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let time = vec![30, 20, 150, 100, 40];
        let result = Solution::num_pairs_divisible_by60(time);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let time = vec![60, 60, 60];
        let result = Solution::num_pairs_divisible_by60(time);
        assert_eq!(result, 3);
    }
}
