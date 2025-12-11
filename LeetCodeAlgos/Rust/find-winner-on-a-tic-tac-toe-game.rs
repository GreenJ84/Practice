// Tic-tac-toe is played by two players A and B on a 3 x 3 grid. The rules of Tic-Tac-Toe are:
// - Players take turns placing characters into empty squares ' '.
// - The first player A always places 'X' characters, while the second player B always places 'O' characters.
// - 'X' and 'O' characters are always placed into empty squares, never on filled ones.
// - The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
// - The game also ends if all squares are non-empty.
// - No more moves can be played if the game is over.

// Given a 2D integer array moves where moves[i] = [rowi, coli] indicates that the ith move will be played on grid[rowi][coli]. return the winner of the game if it exists (A or B). In case the game ends in a draw return "Draw". If there are still movements to play return "Pending".

// You can assume that moves is valid (i.e., it follows the rules of Tic-Tac-Toe), the grid is initially empty, and A will play first.

struct Solution;
impl Solution {
  pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    if moves.len() < 5 { return "Pending".to_string(); }
    let mut board = [[0usize; 3]; 3];

    for turns in 0..moves.len(){
      let (player, choice) = (turns % 2 + 1, [moves[turns][0] as usize, moves[turns][1] as usize]);
      board[choice[0]][choice[1]] = player;
      println!("Turn: {} Player: {} Choice: {:?}", turns, player, choice);
      println!("Board: {:?}", board);

      if turns >= 4 && Self::check_board(&board, player, choice[0], choice[1]) {
        return if player == 1 { "A".to_string() } else { "B".to_string() }
      }
    }
    return if moves.len() < 9 { "Pending".to_string() } else { "Draw".to_string() }
  }

  fn check_board(board: &[[usize; 3]; 3], player: usize, choice_row: usize, choice_col: usize) -> bool{
    for row in 0..3{
      println!("Checking board col for player {}", player);
      println!("Checking board col for player {} - {},{}", player, row, choice_col);
      if board[row][choice_col] != player { break; }
      if row == 2 { return true; }
    }
    println!("Checking board row for player {}", player);
    for col in 0..3{
      println!("Checking board row for player {} - {},{}", player, choice_row, col);
      if board[choice_row][col] != player { break; }
      if col == 2 { return true; }
    }
    println!("Checking board diagonals for player {}", player);
    if choice_row == choice_col {
      for cross in 0..3{
        println!("Checking board main diagonal for player {} - {},{}", player, cross, cross);
        if board[cross][cross] != player { break; }
        if cross == 2 { return true; }
      }
    }
    if choice_row + choice_col == 2 {
      println!("Checking board anti-diagonal for player {} - {},{}", player, choice_row, choice_col);
      for cross in 0..3{
        if board[cross][2-cross] != player { break; }
        if cross == 2 { return true; }
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  // Existing examples
  #[test]
  fn example_1_a_wins_diagonal() {
    let moves = vec![vec![0,0], vec![2,0], vec![1,1], vec![2,1], vec![2,2]];
    assert_eq!(Solution::tictactoe(moves), "A");
  }

  #[test]
  fn example_2_b_wins_column() {
    let moves = vec![vec![0,0], vec![1,1], vec![0,1], vec![0,2], vec![1,0], vec![2,0]];
    assert_eq!(Solution::tictactoe(moves), "B");
  }

  #[test]
  fn example_3_draw() {
    let moves = vec![
      vec![0,0], vec![1,1], vec![2,0],
      vec![1,0], vec![1,2], vec![2,1],
      vec![0,1], vec![0,2], vec![2,2]
    ];
    assert_eq!(Solution::tictactoe(moves), "Draw");
  }

  // Additional tests

  // Pending with minimal moves (only first move)
  #[test]
  fn pending_minimal() {
    let moves = vec![vec![0, 0]];
    assert_eq!(Solution::tictactoe(moves), "Pending");
  }

  // Pending mid-game (5 moves, no winner yet)
  #[test]
  fn pending_mid_game() {
    let moves = vec![vec![0,0], vec![1,1], vec![0,2], vec![2,2], vec![2,0]];
    assert_eq!(Solution::tictactoe(moves), "Pending");
  }

  // A wins by first row
  #[test]
  fn a_wins_row() {
    let moves = vec![vec![0,0], vec![1,0], vec![0,1], vec![1,1], vec![0,2]];
    assert_eq!(Solution::tictactoe(moves), "A");
  }

  // B wins by second column
  #[test]
  fn b_wins_column() {
    let moves = vec![vec![0,0], vec![0,1], vec![2,0], vec![1,1], vec![1,2], vec![2,1]];
    assert_eq!(Solution::tictactoe(moves), "B");
  }

  // A wins main diagonal [0,0],[1,1],[2,2]
  #[test]
  fn a_wins_main_diagonal() {
    let moves = vec![vec![0,0], vec![0,1], vec![1,1], vec![2,1], vec![2,2]];
    assert_eq!(Solution::tictactoe(moves), "A");
  }

  // B wins anti-diagonal [0,2],[1,1],[2,0]
  #[test]
  fn b_wins_anti_diagonal() {
    let moves = vec![vec![0,0], vec![0,2], vec![1,0], vec![1,1], vec![2,1], vec![2,0]];
    assert_eq!(Solution::tictactoe(moves), "B");
  }

  // Last move results in a win for A (not draw)
  #[test]
  fn a_wins_on_last_move() {
    let moves = vec![
      vec![0,0], vec![0,1], vec![1,1], vec![2,1],
      vec![2,2], vec![1,0], vec![0,2], vec![1,2], vec![2,0] // A completes column 0 at the end
    ];
    assert_eq!(Solution::tictactoe(moves), "A");
  }

  // Full board draw with different pattern
  #[test]
  fn full_board_draw_variant() {
    let moves = vec![
      vec![0,1], vec![0,0], vec![0,2],
      vec![1,1], vec![1,0], vec![2,1],
      vec![2,2], vec![1,2], vec![2,0]
    ];
    assert_eq!(Solution::tictactoe(moves), "Draw");
  }

  // Early A win preventing further moves (A wins in 5 moves)
  #[test]
  fn early_a_win_prevents_more() {
    let moves = vec![vec![2,0], vec![0,0], vec![2,1], vec![1,0], vec![2,2]];
    assert_eq!(Solution::tictactoe(moves), "A");
  }

  // Early B win in 6 moves
  #[test]
  fn early_b_win_in_six() {
    let moves = vec![vec![1,2], vec![0,0], vec![2,2], vec![1,0], vec![0,1], vec![2,0]];
    assert_eq!(Solution::tictactoe(moves), "B");
  }
}
