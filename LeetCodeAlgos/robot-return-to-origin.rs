// There is a robot starting at the position (0, 0), the origin, on a 2D plane. Given a sequence of its moves, judge if this robot ends up at (0, 0) after it completes its moves.

// You are given a string moves that represents the move sequence of the robot where moves[i] represents its ith move. Valid moves are 'R' (right), 'L' (left), 'U' (up), and 'D' (down).

// Return true if the robot returns to the origin after it finishes all of its moves, or false otherwise.

// Note: The way that the robot is "facing" is irrelevant. 'R' will always make the robot move to the right once, 'L' will always make it move left, etc. Also, assume that the magnitude of the robot's movement is the same for each move.

struct Solution {}
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut coord = [0, 0];
        for c in moves.chars() {
            match c {
                'R' => coord[0] += 1,
                'L' => coord[0] -= 1,
                'U' => coord[1] += 1,
                'D' => coord[1] -= 1,
                _ => return false,
            }
        }
        coord == [0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_circle() {
        assert_eq!(Solution::judge_circle("UD".to_string()), true);
    }

    #[test]
    fn test_judge_circle_2() {
        assert_eq!(Solution::judge_circle("LL".to_string()), false);
    }
}