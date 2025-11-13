// You are given an 8 x 8 matrix representing a chessboard. There is exactly one white rook represented by 'R', some number of white bishops 'B', and some number of black pawns 'p'. Empty squares are represented by '.'.

// A rook can move any number of squares horizontally or vertically (up, down, left, right) until it reaches another piece or the edge of the board. A rook is attacking a pawn if it can move to the pawn's square in one move.

// Note: A rook cannot move through other pieces, such as bishops or pawns. This means a rook cannot attack a pawn if there is another piece blocking the path.

// Return the number of pawns the white rook is attacking.

struct Solution;
impl Solution {
  pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    for row in 0..8 {
      for col in 0..8 {
        if board[row][col] == 'R' {
          let mut captures = 0;

          // Check upwards
          for r in (0..row).rev() {
            match board[r][col] {
              'B' => break,
              'p' => {
                captures += 1;
                break;
              }
              _ => continue,
            }
          }

          // Check downwards
          for r in row + 1..8 {
            match board[r][col] {
              'B' => break,
              'p' => {
                captures += 1;
                break;
              }
              _ => continue,
            }
          }

          // Check leftwards
          for c in (0..col).rev() {
            match board[row][c] {
              'B' => break,
              'p' => {
                captures += 1;
                break;
              }
              _ => continue,
            }
          }

          // Check rightwards
          for c in col + 1..8 {
            match board[row][c] {
              'B' => break,
              'p' => {
                captures += 1;
                break;
              }
              _ => continue,
            }
          }

          return captures;
        }
      }
    }
    0
  }
}