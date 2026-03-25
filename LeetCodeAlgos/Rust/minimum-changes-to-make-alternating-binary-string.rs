// You are given a string s consisting only of the characters '0' and '1'. In one operation, you can change any '0' to '1' or vice versa.

// The string is called alternating if no two adjacent characters are equal. For example, the string "010" is alternating, while the string "0100" is not.

// Return the minimum number of operations needed to make s alternating.

struct Solution;
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut starts = [0, 0];
        let mut temp = '0';

        for ch in s.chars() {
          if ch == temp {
            starts[0] += 1;
          } else {
            starts[1] += 1;
          }
          temp = match temp {
            '1' => '0',
            _ => '1'
          };
        }
        starts[0].min(starts[1])
    }

    //# FAILED EDGE CASES
    pub fn _min_operations1(s: String) -> i32 {
        let (n, s)= (s.len(), s.chars().collect::<Vec<_>>());

        let (mut last, mut ops) = (s[0], 0);
        for i in 1..n {
          if s[i] == last {
            ops += 1;
            last = if s[i] == '0' { '1' } else { '0' };
          } else {
            last = s[i];
          }
        }

        let first = ops;
        (last, ops) = (s[n-1], 0);
        for i in n-2..=0 {
          if s[i] == last {
            ops += 1;
            last = if s[i] == '0' { '1' } else { '0' };
          } else {
            last = s[i];
          }
        }
        first.min(ops)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "0100".to_string();
        assert_eq!(Solution::min_operations(s), 1);
    }

    #[test]
    fn test2() {
        let s = "10".to_string();
        assert_eq!(Solution::min_operations(s), 0);
    }

    #[test]
    fn test3() {
        let s = "1111".to_string();
        assert_eq!(Solution::min_operations(s), 2);
    }
}

// Example 1:

// Input: s = "0100"
// Output: 1
// Explanation: If you change the last character to '1', s will be "0101", which is alternating.
// Example 2:

// Input: s = "10"
// Output: 0
// Explanation: s is already alternating.
// Example 3:

// Input: s = "1111"
// Output: 2
// Explanation: You need two operations to reach "0101" or "1010".
