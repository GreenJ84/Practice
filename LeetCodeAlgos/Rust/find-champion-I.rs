// There are n teams numbered from 0 to n - 1 in a tournament.

// Given a 0-indexed 2D boolean matrix grid of size n * n. For all i, j that 0 <= i, j <= n - 1 and i != j team i is stronger than team j if grid[i][j] == 1, otherwise, team j is stronger than team i.

// Team a will be the champion of the tournament if there is no team b that is stronger than team a.

// Return the team that will be the champion of the tournament.

// Constraints:
// n == grid.length
// n == grid[i].length
// 2 <= n <= 100
// grid[i][j] is either 0 or 1.
// For all i grid[i][i] is 0.
// For all i, j that i != j, grid[i][j] != grid[j][i].
// The input is generated such that if team a is stronger than team b and team b is stronger than team c, then team a is stronger than team c.

struct Solution;
impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut champion = 0;

        for i in 1..n {
            if grid[i][champion] == 1 {
                champion = i;
            }
        }

        champion as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(Solution::find_champion(grid), 0);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]];
        assert_eq!(Solution::find_champion(grid), 1);
    }
}
