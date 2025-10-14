// You are given a 0-indexed 2D matrix grid of size m x n, where (r, c) represents:

// A land cell if grid[r][c] = 0, or
// A water cell containing grid[r][c] fish, if grid[r][c] > 0.
// A fisher can start at any water cell (r, c) and can do the following operations any number of times:

// Catch all the fish at cell (r, c), or
// Move to any adjacent water cell.
// Return the maximum number of fish the fisher can catch if he chooses his starting cell optimally, or 0 if no water cell exists.

// An adjacent cell of the cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) or (r - 1, c) if it exists.

struct Solution;
impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;
        for row in 0..m {
          for col in 0..n {
            if grid[row][col] > 0 {
              let fish = Self::check_fishing(&mut grid, row, col, 0);
              if fish > ans { ans = fish; }
            }
          }
        }
        ans
    }

    fn check_fishing(grid: &mut Vec<Vec<i32>>, row: usize, col: usize, mut fish: i32) -> i32 {
      fish += grid[row][col];
      grid[row][col] = 0;
      let dirs: [(i8, i8); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
      for (x, y) in dirs {
        if row as i8 + x < 0 || row as i8 + x >= grid.len() as i8 { continue; }
        if col as i8 + y < 0 || col as i8 + y >= grid[0].len() as i8 { continue; }
        if grid[row + x as usize][col + y as usize] > 0 {
          fish += Self::check_fishing(grid, row + x as usize, col + y as usize, 0);
        }
      }
      fish
    }
}