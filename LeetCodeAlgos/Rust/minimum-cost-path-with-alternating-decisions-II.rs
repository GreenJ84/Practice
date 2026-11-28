// You are given two integers m and n representing the number of rows and columns of a grid, respectively.

// The cost to enter cell (i, j) is defined as (i + 1) * (j + 1).

// You are also given a 2D integer array waitCost where waitCost[i][j] defines the cost to wait on that cell.

// The path will always begin by entering cell (0, 0) on move 1 and paying the entrance cost.

// At each step, you follow an alternating pattern:

// On odd-numbered seconds, you must move right or down to an adjacent cell, paying its entry cost.
// On even-numbered seconds, you must wait in place for exactly one second and pay waitCost[i][j] during that second.
// Return the minimum total cost required to reach (m - 1, n - 1).

// Constraints:

// 1 <= m, n <= 10^5
// 2 <= m * n <= 10^5
// waitCost.length == m
// waitCost[0].length == n
// 0 <= waitCost[i][j] <= 10^5

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn min_cost(m: i32, n: i32, wait_cost: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![i64::MAX; n]; m];
        let w_cost = |i: usize, j: usize| -> i64 {
            return wait_cost[i][j] as i64;
        };
        let m_cost = |i: usize, j: usize| -> i64 {
            return ((i + 1) * (j + 1)) as i64;
        };

        let mut queue = VecDeque::new();
        queue.push_back((0usize, 0usize));
        while !queue.is_empty() {
            if let Some(spot) = queue.pop_front() {
                let (i, j) = spot;

                let cost = m_cost(i, j);
                let seen = dp[i][j] != i64::MAX;

                if i == 0 && j == 0 {
                    dp[i][j] = cost;
                } else {
                    let mut cost = cost;
                    if i < m - 1 || j < n - 1 {
                        cost += w_cost(i, j);
                    }

                    if i > 0 {
                        dp[i][j] = dp[i][j].min(dp[i - 1][j] + cost);
                    }
                    if j > 0 {
                        dp[i][j] = dp[i][j].min(dp[i][j - 1] + cost);
                    }
                }

                if !seen && i < m - 1 {
                    queue.push_back((i + 1, j));
                }
                if !seen && j < n - 1 {
                    queue.push_back((i, j + 1));
                }
            }
        }
        dp[m - 1][n - 1]
    }

    // Time Limit Exceeded
    pub fn _min_cost1(m: i32, n: i32, wait_cost: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![i64::MAX; n]; m];
        let w_cost = |i: usize, j: usize| -> i64 {
            return wait_cost[i][j] as i64;
        };
        let m_cost = |i: usize, j: usize| -> i64 {
            return ((i + 1) * (j + 1)) as i64;
        };

        let mut queue = VecDeque::new();
        queue.push_back((0usize, 0usize));
        while !queue.is_empty() {
            if let Some(spot) = queue.pop_front() {
                let (i, j) = spot;
                let cost = m_cost(i, j);
                if i == 0 && j == 0 {
                    dp[i][j] = cost;
                } else {
                    let mut cost = cost;
                    if i < m - 1 || j < n - 1 {
                        cost += w_cost(i, j);
                    }
                    let mut min_cost = i64::MAX;
                    if i > 0 {
                        min_cost = min_cost.min(dp[i - 1][j] + cost);
                    }
                    if j > 0 {
                        min_cost = min_cost.min(dp[i][j - 1] + cost);
                    }
                    dp[i][j] = min_cost;
                }

                if i < m - 1 {
                    queue.push_back((i + 1, j));
                }
                if j < n - 1 {
                    queue.push_back((i, j + 1));
                }
            }
        }
        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let m = 1;
        let n = 2;
        let wait_cost = vec![vec![1, 2]];
        assert_eq!(Solution::min_cost(m, n, wait_cost), 3);
    }

    #[test]
    fn test_2() {
        let m = 2;
        let n = 2;
        let wait_cost = vec![vec![3, 5], vec![2, 4]];
        assert_eq!(Solution::min_cost(m, n, wait_cost), 9);
    }

    #[test]
    fn test_3() {
        let m = 2;
        let n = 3;
        let wait_cost = vec![vec![6, 1, 4], vec![3, 2, 5]];
        assert_eq!(Solution::min_cost(m, n, wait_cost), 16);
    }
}
