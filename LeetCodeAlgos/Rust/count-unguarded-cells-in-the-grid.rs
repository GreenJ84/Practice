// You are given two integers m and n representing a 0-indexed m x n grid. You are also given two 2D integer arrays guards and walls where guards[i] = [rowi, coli] and walls[j] = [rowj, colj] represent the positions of the ith guard and jth wall respectively.

// A guard can see every cell in the four cardinal directions (north, east, south, or west) starting from their position unless obstructed by a wall or another guard. A cell is guarded if there is at least one guard that can see it.

// Return the number of unoccupied cells that are not guarded.

struct Solution;
impl Solution {
  pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let mut grid = vec![vec![0i32; n as usize]; m as usize];
    // mark walls
    for wall in &walls {
      grid[wall[0] as usize][wall[1] as usize] = -1;
    }
    // mark guards as obstacles too
    for guard in &guards {
      grid[guard[0] as usize][guard[1] as usize] = -1;
    }

    // mark guarded cells from each guard
    for pair in &guards {
      let row = pair[0];
      let col = pair[1];

      // Up
      let mut new = row - 1;
      while new >= 0 &&
      grid[new as usize][col as usize] != -1 {
        grid[new as usize][col as usize] = 1;
        new -= 1;
      }
      // Down
      new = row + 1;
      while new < m &&
      grid[new as usize][col as usize] != -1 {
        grid[new as usize][col as usize] = 1;
        new += 1;
      }
      // Left
      new = col - 1;
      while new >= 0 &&
      grid[row as usize][new as usize] != -1 {
        grid[row as usize][new as usize] = 1;
        new -= 1;
      }
      // Right
      new = col + 1;
      while new < n &&
      grid[row as usize][new as usize] != -1 {
        grid[row as usize][new as usize] = 1;
        new += 1;
      }
    }

    grid.iter()
      .map(|row| {
        row.iter().filter(|&c| *c == 0).count()
      })
      .sum::<usize>() as i32
  }
}