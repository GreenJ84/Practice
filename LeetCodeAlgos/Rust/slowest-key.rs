// A newly designed keypad was tested, where a tester pressed a sequence of n keys, one at a time.

// You are given a string keysPressed of length n, where keysPressed[i] was the ith key pressed in the testing sequence, and a sorted list releaseTimes, where releaseTimes[i] was the time the ith key was released. Both arrays are 0-indexed. The 0th key was pressed at the time 0, and every subsequent key was pressed at the exact time the previous key was released.

// The tester wants to know the key of the keypress that had the longest duration. The ith keypress had a duration of releaseTimes[i] - releaseTimes[i - 1], and the 0th keypress had a duration of releaseTimes[0].

// Note that the same key could have been pressed multiple times during the test, and these multiple presses of the same key may not have had the same duration.

// Return the key of the keypress that had the longest duration. If there are multiple such keypresses, return the lexicographically largest key of the keypresses.

// Constraints:
// releaseTimes.length == n
// keysPressed.length == n
// 2 <= n <= 1000
// 1 <= releaseTimes[i] <= 109
// releaseTimes[i] < releaseTimes[i+1]
// keysPressed contains only lowercase English letters.

struct Solution;
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut keys = keys_pressed.chars().enumerate();
        let mut longest: (char, i32) = (keys.next().unwrap().1, release_times[0]);
        for (i, key) in keys {
            let dur = release_times[i] - release_times[i - 1];
            if dur > longest.1 || (dur == longest.1 && key > longest.0) {
                longest = (key, dur);
            }
        }
        longest.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let release_times = vec![9, 29, 49, 50];
        let keys_pressed = "cbcd".to_string();
        let result = Solution::slowest_key(release_times, keys_pressed);
        assert_eq!(result, 'c');
    }

    #[test]
    fn test_2() {
        let release_times = vec![12, 23, 36, 46, 62];
        let keys_pressed = "spuda".to_string();
        let result = Solution::slowest_key(release_times, keys_pressed);
        assert_eq!(result, 'a');
    }
}
