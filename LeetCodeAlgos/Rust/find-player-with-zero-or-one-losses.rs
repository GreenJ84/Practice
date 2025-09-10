// You are given an integer array matches where matches[i] = [winneri, loseri] indicates that the player winneri defeated player loseri in a match.

// Return a list answer of size 2 where:

// answer[0] is a list of all players that have not lost any matches.
// answer[1] is a list of all players that have lost exactly one match.
// The values in the two lists should be returned in increasing order.

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut player_losses: HashMap<i32, i32> = HashMap::new();

        for mat in matches {
          let winner = mat[0];
          let loser = mat[1];

          match (player_losses.contains_key(&winner), player_losses.contains_key(&loser)) {
            (true, true) => {
              player_losses.entry(loser).and_modify(|losses| *losses += 1);
            },
            (true, false) => {
              player_losses.insert(loser, 1);
            },
            (false, true) => {
              player_losses.insert(winner, 0);
              player_losses.entry(loser).and_modify(|losses| *losses += 1);
            }
            _ => {
              player_losses.insert(loser, 1);
              player_losses.insert(winner, 0);
            }
          }
        }
        let mut no_losses: Vec<i32> = vec![];
        let mut one_loss: Vec<i32> = vec![];

        for (player, losses) in player_losses {
          if losses == 0 {
            no_losses.push(player);
          } else if losses == 1 {
            one_loss.push(player);
          }
        }

        no_losses.sort();
        one_loss.sort();
        vec![
          no_losses,
          one_loss
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_winners() {
        let matches = vec![vec![1, 3], vec![2, 3], vec![3, 6], vec![5, 6], vec![5, 7], vec![4, 5], vec![4, 8], vec![4, 9], vec![10, 4], vec![10, 9]];
        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(Solution::find_winners(matches), expected);
    }

    #[test]
    fn test_find_winners_no_losses() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let expected = vec![vec![1, 2, 5, 6], vec![]];
        assert_eq!(Solution::find_winners(matches), expected);
    }
}

