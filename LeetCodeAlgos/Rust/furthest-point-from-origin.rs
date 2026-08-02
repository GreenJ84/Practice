// You are given a string moves of length n consisting only of characters 'L', 'R', and '_'. The string represents your movement on a number line starting from the origin 0.

// In the ith move, you can choose one of the following directions:

// move to the left if moves[i] = 'L' or moves[i] = '_'
// move to the right if moves[i] = 'R' or moves[i] = '_'
// Return the distance from the origin of the furthest point you can get to after n moves.

// Constraints:
// 1 <= moves.length == n <= 50
// moves consists only of characters 'L', 'R' and '_'.

struct Solution;
impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (mut dist, mut ext) = (0i32, 0i32);
        for ch in moves.chars() {
            match ch {
                'L' => {
                    dist -= 1;
                }
                'R' => {
                    dist += 1;
                }
                _ => {
                    ext += 1;
                }
            }
        }
        dist.abs() + ext
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let moves = String::from("L_RL__R");
        assert_eq!(Solution::furthest_distance_from_origin(moves), 3);
    }

    #[test]
    fn test_2() {
        let moves = String::from("_R__LL_");
        assert_eq!(Solution::furthest_distance_from_origin(moves), 5);
    }

    #[test]
    fn test_3() {
        let moves = String::from("_______");
        assert_eq!(Solution::furthest_distance_from_origin(moves), 7);
    }
}
