// Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.

// A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).

// Constraints:
// - n == grid.length == grid[i].length
// - 1 <= n <= 200
// - 1 <= grid[i][j] <= 105

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let col_starts = grid[0].iter().enumerate().fold(
            HashMap::<i32, Vec<usize>>::new(),
            |mut acc, (col, &val)| {
                acc.entry(val).or_default().push(col);
                acc
            },
        );

        let mut ans = 0i32;
        for row in 0..n {
            if let Some(cols) = col_starts.get(&grid[row][0]) {
                for &col in cols {
                    if Self::pair_dp((row, 1), (1, col), &grid, n) {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }

    fn pair_dp(row: (usize, usize), col: (usize, usize), grid: &Vec<Vec<i32>>, n: usize) -> bool {
        for df in 0..n - 1 {
            if grid[row.0][row.1 + df] != grid[col.0 + df][col.1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        assert_eq!(Solution::equal_pairs(grid), 1);
    }

    #[test]
    fn test2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        assert_eq!(Solution::equal_pairs(grid), 3);
    }
}
