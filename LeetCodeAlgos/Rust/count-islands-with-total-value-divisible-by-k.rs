// You are given an m x n matrix grid and a positive integer k. An island is a group of positive integers (representing land) that are 4-directionally connected (horizontally or vertically).

// The total value of an island is the sum of the values of all cells in the island.

// Return the number of islands with a total value divisible by k.

struct Solution;
impl Solution {
  pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());

    let mut ans = 0;
    for row in 0..m {
      for col in 0..n {
        if grid[row][col] != 0 &&
        Self::island_value(&mut grid, row, col) % k == 0 {
          ans += 1;
        }
      }
    }
    ans
  }

  fn island_value(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut sum = grid[row][col];
    grid[row][col] = 0;

    for (y, x) in [(1, 0), (0, 1)] {
      // Bottom, Right
      let (new_row, new_col) = (row + y, col + x);
      if (new_row < grid.len()) &&
      (new_col < grid[0].len()) &&
      grid[new_row][new_col] != 0 {
        sum += Self::island_value(grid, new_row, new_col);
      }

      if let (Some(new_row), Some(new_col)) = (row.checked_sub(y), col.checked_sub(x)) {
        if grid[new_row][new_col] != 0 {
          sum += Self::island_value(grid, new_row, new_col);
        }
      }
    }
    sum
  }
}