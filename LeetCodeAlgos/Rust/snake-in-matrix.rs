// There is a snake in an n x n matrix grid and can move in four possible directions. Each cell in the grid is identified by the position: grid[i][j] = (i * n) + j.

// The snake starts at cell 0 and follows a sequence of commands.

// You are given an integer n representing the size of the grid and an array of strings commands where each command[i] is either "UP", "RIGHT", "DOWN", and "LEFT". It's guaranteed that the snake will remain within the grid boundaries throughout its movement.

// Return the position of the final cell where the snake ends up after executing commands.

// Constraints:

// 2 <= n <= 10
// 1 <= commands.length <= 100
// commands consists only of "UP", "RIGHT", "DOWN", and "LEFT".
// The input is generated such the snake will not move outside of the boundaries.

struct Solution;
impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let (mut row, mut col) = (0i32, 0i32);
        for command in &commands {
          match command.as_str() {
            "RIGHT" => {
              col += 1;
            },
            "DOWN" => {
              row += 1;
            },
            "LEFT" => {
              col -= 1;
            },
            _ => {
              row -= 1;
            }
          }
        }
        row * n + col
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let n = 2;
    let commands = vec!["RIGHT".into(), "DOWN".into()];
    let result = Solution::final_position_of_snake(n, commands);
    assert_eq!(result, 3);
  }

  #[test]
  fn test_2() {
    let n = 3;
    let commands = vec!["DOWN".into(), "RIGHT".into(), "UP".into()];
    let result = Solution::final_position_of_snake(n, commands);
    assert_eq!(result, 1);
  }

  #[test]
  fn test_3() {
    let n = 4;
    let commands = vec!["RIGHT".into(), "DOWN".into()];
    let result = Solution::final_position_of_snake(n, commands);
    assert_eq!(result, 5);
  }
}
