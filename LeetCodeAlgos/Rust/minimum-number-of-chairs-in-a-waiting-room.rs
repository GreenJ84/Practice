// You are given a string s. Simulate events at each second i:

// If s[i] == 'E', a person enters the waiting room and takes one of the chairs in it.
// If s[i] == 'L', a person leaves the waiting room, freeing up a chair.
// Return the minimum number of chairs needed so that a chair is available for every person who enters the waiting room given that it is initially empty.

struct Solution;
impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut chairs = 0;
        let mut max_chairs = 0;

        for c in s.chars() {
            if c == 'E' {
                chairs += 1;
                max_chairs = max_chairs.max(chairs);
            } else if c == 'L' {
                chairs -= 1;
            }
        }

        max_chairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "EEEEEEE".to_string();
        let result = Solution::minimum_chairs(s);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_2() {
        let s = "ELELEEL".to_string();
        let result = Solution::minimum_chairs(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let s = "ELEELEELLL".to_string();
        let result = Solution::minimum_chairs(s);
        assert_eq!(result, 3);
    }
}
