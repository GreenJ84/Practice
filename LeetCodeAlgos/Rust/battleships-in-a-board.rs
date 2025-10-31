// Given an m x n matrix board where each cell is a battleship 'X' or empty '.', return the number of the battleships on board.

// Battleships can only be placed horizontally or vertically on board. In other words, they can only be made of the shape 1 x k (1 row, k columns) or k x 1 (k rows, 1 column), where k can be of any size. At least one horizontal or vertical cell separates between two battleships (i.e., there are no adjacent battleships).

struct Solution;
impl Solution {
  pub fn count_battleships(mut board: Vec<Vec<char>>) -> i32 {
    let (m, n) = (board.len(), board[0].len());
    let mut ans = 0;

    for row in (0..m).rev() {
      for col in (0..n).rev() {
        if board[row][col] == 'X' {
          ans += 1;
          Self::remove_ship(&mut board, row, col);
        }
      }
    }
    ans
  }

  pub fn remove_ship(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
    board[row][col] = '.';
    for (x, y) in [(0, 1), (1, 0)] {
      if row.checked_sub(x).is_none() || col.checked_sub(y).is_none() { continue; }
      if board[row - x][col - y] == 'X' {
        Self::remove_ship(board, row - x, col - y);
      }
    }
  }
}