// You are given two 0-indexed integer arrays player1 and player2, that represent the number of pins that player 1 and player 2 hit in a bowling game, respectively.

// The bowling game consists of n turns, and the number of pins in each turn is exactly 10.

// Assume a player hit xi pins in the ith turn. The value of the ith turn for the player is:

// 2xi if the player hit 10 pins in any of the previous two turns.
// Otherwise, It is xi.
// The score of the player is the sum of the values of their n turns.

// Return

// 1 if the score of player 1 is more than the score of player 2,
// 2 if the score of player 2 is more than the score of player 1, and
// 0 in case of a draw.


struct Solution {}
impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut p1: [i32; 2] = [0, -5];
        let mut p2: [i32; 2] = [0, -5];
        for i in 0..player1.len() {
            if i as i32 - p1[1] == 1 || i as i32 - p1[1] == 2 {
                p1[0] += player1[i] * 2;
            } else {
                p1[0] += player1[i]
            }
            if player1[i] == 10 { p1[1] = i as i32; }

            if i as i32 - p2[1] == 1 || i as i32 - p2[1] == 2 {
                p2[0] += player2[i] * 2;
            } else {
                p2[0] += player2[i]
            }
            if player2[i] == 10 { p2[1] = i as i32; }
            print!("{} {}", p1[0], p2[0])
        }

        print!("{} {}", p1[0], p2[0]);
        if p1[0] == p2[0] {
            0
        } else if p1 > p2 {
            1
        } else {
            2
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_winner() {
        assert_eq!(Solution::is_winner(vec![4,10,7,9], vec![6,5,2,3]), 1);
    }

    #[test]
    fn test_is_winner2() {
        assert_eq!(Solution::is_winner(vec![3,5,7,6], vec![8,10,10,2]), 2);
    }

    #[test]
    fn test_is_winner3() {
        assert_eq!(Solution::is_winner(vec![2,3], vec![4,1]), 0);
    }

    #[test]
    fn test_is_winner4() {
        assert_eq!(Solution::is_winner(vec![4,1,6,6,9], vec![1,7,4,10,2]), 0);
    }
}