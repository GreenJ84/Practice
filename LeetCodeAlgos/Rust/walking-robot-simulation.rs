// A robot on an infinite XY-plane starts at point (0, 0) facing north. The robot receives an array of integers commands, which represents a sequence of moves that it needs to execute. There are only three possible types of instructions the robot can receive:

// -2: Turn left 90 degrees.
// -1: Turn right 90 degrees.
// 1 <= k <= 9: Move forward k units, one unit at a time.
// Some of the grid squares are obstacles. The ith obstacle is at grid point obstacles[i] = (xi, yi). If the robot runs into an obstacle, it will stay in its current location (on the block adjacent to the obstacle) and move onto the next command.

// Return the maximum squared Euclidean distance that the robot reaches at any point in its path (i.e. if the distance is 5, return 25).

// Note:

// There can be an obstacle at (0, 0). If this happens, the robot will ignore the obstacle until it has moved off the origin. However, it will be unable to return to (0, 0) due to the obstacle.
// North means +Y direction.
// East means +X direction.
// South means -Y direction.
// West means -X direction.

struct Solution;

const DIRS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles = obstacles
            .into_iter()
            .map(|x| (x[0], x[1]))
            .collect::<std::collections::HashSet<(i32, i32)>>();

        let (mut location, mut dir) = ((0i32, 0i32), 0usize);
        let mut furthest_point = 0i32;
        for command in commands {
            match command {
                n if n > 0 && n < 10 => {
                    for _ in 0..n {
                        let next_location = (location.0 + DIRS[dir][0], location.1 + DIRS[dir][1]);
                        if obstacles.contains(&(next_location.0, next_location.1)) {
                            break;
                        }
                        location = (next_location.0, next_location.1);
                    }
                    furthest_point = furthest_point.max((location.0).pow(2) + (location.1).pow(2));
                }
                -2 => {
                    dir = if dir == 0 { 3 } else { dir - 1 };
                }
                _ => {
                    dir = (dir + 1) % 4;
                }
            }
        }
        furthest_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let commands = vec![4, -1, 3];
        let obstacles = vec![];
        assert_eq!(Solution::robot_sim(commands, obstacles), 25);
    }

    #[test]
    fn test2() {
        let commands = vec![4, -1, 4, -2, 4];
        let obstacles = vec![vec![2, 4]];
        assert_eq!(Solution::robot_sim(commands, obstacles), 65);
    }

    #[test]
    fn test3() {
        let commands = vec![6, -1, -1, 6];
        let obstacles = vec![vec![0, 0]];
        assert_eq!(Solution::robot_sim(commands, obstacles), 36);
    }

    #[test]
    fn test4() {
        let commands = vec![7, -2, -2, 7, 5];
        let obstacles = vec![];
        assert_eq!(Solution::robot_sim(commands, obstacles), 49);
    }
}
