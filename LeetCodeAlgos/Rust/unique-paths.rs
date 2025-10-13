// There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.

// Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.

// The test cases are generated so that the answer will be less than or equal to 2 * 109.

struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m  {
            dp[i][0] = 1;
        }
        for i in 1..n  {
            dp[0][i] = 1;
        }
        for row in 1..m {
            for col in 1..n {
                dp[row][col] = dp[row - 1][col] + dp[row][col - 1];
            }
        }
        dp[m-1][n-1]
    }
}