// You are given an n x n binary matrix grid where 1 represents land and 0 represents water.

// An island is a 4-directionally connected group of 1's not connected to any other 1's. There are exactly two islands in grid.

// You may change 0's to 1's to connect the two islands to form one island.

// Return the smallest number of 0's you must flip to connect the two islands.

// Constraints:
// - n == grid.length == grid[i].length
// - 2 <= n <= 100
// - grid[i][j] is either 0 or 1.
// - There are exactly two islands in grid.

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
      let n = grid.len();

      let mut land1 = Vec::new();
      let mut bfs_search = VecDeque::new();
      'outer: for row in 0..n {
        for col in 0..n {
          if grid[row][col] == 1 {
            grid[row][col] = 2;
            land1.push((row, col));
            bfs_search.push_back((row, col, 0));
            break 'outer;
          }
        }
      }

      while let Some((l_row, l_col)) = land1.pop() {
        if l_row > 0 && grid[l_row - 1][l_col] == 1  {
            grid[l_row - 1][l_col] = 2;
            land1.push((l_row - 1, l_col));
            bfs_search.push_back((l_row - 1, l_col, 0));
        }
        if l_row < n - 1 && grid[l_row + 1][l_col] == 1 {
            grid[l_row + 1][l_col] = 2;
            land1.push((l_row + 1, l_col));
            bfs_search.push_back((l_row + 1, l_col, 0));
        }
        if l_col > 0 && grid[l_row][l_col - 1] == 1 {
            grid[l_row][l_col - 1] = 2;
            land1.push((l_row, l_col - 1));
            bfs_search.push_back((l_row, l_col - 1, 0));
        }
        if l_col < n - 1 && grid[l_row][l_col + 1] == 1 {
            grid[l_row][l_col + 1] = 2;
            land1.push((l_row, l_col + 1));
            bfs_search.push_back((l_row, l_col + 1, 0));
        }
      }

      while let Some((row, col, dist)) = bfs_search.pop_front() {
        if row > 0 && grid[row - 1][col] != 2 {
            if grid[row - 1][col] == 1 {
                return dist;
            }
            grid[row - 1][col] = 2;
            bfs_search.push_back((row - 1, col, dist + 1));
        }
        if row < n - 1 && grid[row + 1][col] != 2 {
            if grid[row + 1][col] == 1 {
                return dist;
            }
            grid[row + 1][col] = 2;
            bfs_search.push_back((row + 1, col, dist + 1));
        }
        if col > 0 && grid[row][col - 1] != 2 {
            if grid[row][col - 1] == 1 {
                return dist;
            }
            grid[row][col - 1] = 2;
            bfs_search.push_back((row, col - 1, dist + 1));
        }
        if col < n - 1 && grid[row][col + 1] != 2 {
            if grid[row][col + 1] == 1 {
                return dist;
            }
            grid[row][col + 1] = 2;
            bfs_search.push_back((row, col + 1, dist + 1));
        }
      }
      -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![vec![0,1], vec![1,0]];
        assert_eq!(Solution::shortest_bridge(grid), 1);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![0,1,0], vec![0,0,0], vec![0,0,1]];
        assert_eq!(Solution::shortest_bridge(grid), 2);
    }

    #[test]
    fn test_3() {
        let grid = vec![
            vec![1,1,1,1,1],
            vec![1,0,0,0,1],
            vec![1,0,1,0,1],
            vec![1,0,0,0,1],
            vec![1,1,1,1,1]
        ];
        assert_eq!(Solution::shortest_bridge(grid), 1);
    }
}