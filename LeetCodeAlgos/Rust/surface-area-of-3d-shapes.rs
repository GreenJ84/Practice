// You are given an n x n grid where you have placed some 1 x 1 x 1 cubes. Each value v = grid[i][j] represents a tower of v cubes placed on top of cell (i, j).

// After placing these cubes, you have decided to glue any directly adjacent cubes to each other, forming several irregular 3D shapes.

// Return the total surface area of the resulting shapes.

// Note: The bottom face of each shape counts toward its surface area.

struct Solution;
impl Solution {
  pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut area = 0;

    for row in 0..n {
      for col in 0..n {
        let stack = grid[row][col];
        // If valid stack, add top and bottom
        if stack > 0 {
          area += 2;
        }
        // If top row, add only the full top side of stack
        if row == 0 {
          area += stack;
        }
        // If any other row, get size difference between upper cell
        else {
          area += (stack - grid[row - 1][col]).abs();
        }
        // If last row, also add full bottom side of stack
        if row == n - 1 {
          area += stack;
        }
        // If left col, add only the full left side of stack
        if col == 0 {
          area += stack;
        }
        // If any other col, get size difference between left cell
        else {
          area += (stack - grid[row][col - 1]).abs();
        }
        // If last col, also add full right side of stack
        if col == n - 1 {
          area += stack;
        }
      }
    }
    area
  }
}