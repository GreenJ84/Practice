// There is an 8 x 8 empty chessboard with 1-indexed rows and columns.

// You are given an array source = [sr, sc] representing the starting position of a bishop, and an array target = [tr, tc] representing the target position.

// In one move, the bishop travels one or more squares along a single diagonal direction, staying within the board.

// Return the minimum number of moves for the bishop to land exactly on target. If it can never reach target, return -1.

// Constraints:​​​​​​​
// source.length == target.length == 2
// 1 <= sr, sc, tr, tc <= 8
// source != target

struct Solution;
impl Solution {
    pub fn min_bishop_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
        let s = source[0] % 2 == source[1] % 2;
        let t = target[0] % 2 == target[1] % 2;
        if s != t {
            return -1;
        }

        if (source[0] - target[0]).abs() == (source[1] - target[1]).abs() {
            return 1;
        }
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_bishop_moves(vec![8, 1], vec![1, 8]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_bishop_moves(vec![4, 2], vec![1, 3]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_bishop_moves(vec![1, 1], vec![3, 4]), -1);
    }
}
