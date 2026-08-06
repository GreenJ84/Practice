// You are given two 0-indexed binary strings s1 and s2, both of length n, and a positive integer x.

// You can perform any of the following operations on the string s1 any number of times:

// Choose two indices i and j, and flip both s1[i] and s1[j]. The cost of this operation is x.
// Choose an index i such that i < n - 1 and flip both s1[i] and s1[i + 1]. The cost of this operation is 1.
// Return the minimum cost needed to make the strings s1 and s2 equal, or return -1 if it is impossible.

// Note that flipping a character means changing it from 0 to 1 or vice-versa.

// Constraints:
// n == s1.length == s2.length
// 1 <= n, x <= 500
// s1 and s2 consist only of the characters '0' and '1'.

struct Solution;
impl Solution {
    pub fn min_operations(s1: String, s2: String, x: i32) -> i32 {
        let n = s1.len();
        let diff = (0..n)
            .filter(|&i| s1[i..i + 1] != s2[i..i + 1])
            .collect::<Vec<usize>>();
        let n = diff.len();
        if n % 2 == 1 {
            return -1;
        } else if n == 0 {
            return 0;
        }

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            if dp[i - 1] != i32::MAX {
                dp[i] = dp[i - 1] + x;
            }

            if i > 1 && dp[i - 2] != i32::MAX {
                let local_cost = (diff[i - 1] - diff[i - 2]) as i32 * 2;
                dp[i] = dp[i].min(dp[i - 2] + local_cost);
            }
        }
        dp[n] / 2
    }

    pub fn min_operations1(s1: String, s2: String, x: i32) -> i32 {
        let n = s1.len();
        let diff = (0..n)
            .filter_map(|i| {
                if s1[i..i + 1] != s2[i..i + 1] {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();
        let n = diff.len();
        if n % 2 == 1 {
            return -1;
        }
        let mut ans = 0;
        let mut i = 1;
        while i < n {
            let dist = (diff[i] - diff[i - 1]) as i32;
            if dist < x {
                ans += dist;
                i += 1;
            }
            i += 1;
        }
        ans + (n as i32 - 2 * ans) * x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s1 = String::from("1100011000");
        let s2 = String::from("0101001010");
        let x = 2;
        assert_eq!(Solution::min_operations(s1, s2, x), 4);
    }

    #[test]
    fn test_2() {
        let s1 = String::from("10110");
        let s2 = String::from("00011");
        let x = 4;
        assert_eq!(Solution::min_operations(s1, s2, x), -1);
    }
}
