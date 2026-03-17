// You are given two positive integers n and k.

// You can choose any bit in the binary representation of n that is equal to 1 and change it to 0.

// Return the number of changes needed to make n equal to k. If it is impossible, return -1.

struct Solution;
impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n == k { return 0; }

        let (mut n, mut k, mut ans) = (n, k, 0);
        while n > 0 || k > 0 {
          match (n % 2, k % 2) {
            (0, 1) => {
              return -1;
            },
            (1, 0) => {
              ans += 1;
            },
            _ => {}
          }
          n = n >> 1;
          k = k >> 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_changes(13, 4), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_changes(21, 21), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_changes(14, 13), -1);
    }
}

// Example 1:

// Input: n = 13, k = 4

// Output: 2

// Explanation:
// Initially, the binary representations of n and k are n = (1101)2 and k = (0100)2.
// We can change the first and fourth bits of n. The resulting integer is n = (0100)2 = k.

// Example 2:

// Input: n = 21, k = 21

// Output: 0

// Explanation:
// n and k are already equal, so no changes are needed.

// Example 3:

// Input: n = 14, k = 13

// Output: -1

// Explanation:
// It is not possible to make n equal to k.

