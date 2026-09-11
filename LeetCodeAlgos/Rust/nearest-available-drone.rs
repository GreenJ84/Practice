// You are given a 2D integer array drones, where drones[i] = [xi, yi, rangei] represents the x-coordinate, y-coordinate, and travel range of the ith drone.

// You are also given an integer array target = [tx, ty], representing the coordinates of the target.

// A drone drones[i] can reach the target if the Manhattan distance between its coordinates and the target coordinates is less than or equal to its rangei.

// Return the index of the reachable drone with the minimum Manhattan distance to the target. If there is a tie, return the smallest index. If no drone can reach the target, return -1.

// Constraints:
// 1 <= drones.length <= 100
// drones[i] = [xi, yi, rangei]
// target = [tx, ty]
// -25 <= xi, yi, tx, ty <= 25
// 1 <= rangei <= 100

struct Solution;
impl Solution {
    pub fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
        let (mut distance, mut drone_i) = (i32::MAX, None);
        for (i, drone) in drones.into_iter().enumerate() {
            let dist = (drone[0] - target[0]).abs() + (drone[1] - target[1]).abs();
            if dist <= drone[2] && dist < distance {
                distance = dist;
                drone_i = Some(i as i32);
            }
        }
        drone_i.unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let drones = vec![vec![0, 0, 8], vec![2, 2, 9]];
        let target = vec![3, 4];
        assert_eq!(Solution::nearest_drone(drones, target), 1);
    }

    #[test]
    fn test_2() {
        let drones = vec![vec![2, 1, 5], vec![4, 4, 5], vec![6, 6, 8]];
        let target = vec![5, 5];
        assert_eq!(Solution::nearest_drone(drones, target), 1);
    }

    #[test]
    fn test_3() {
        let drones = vec![vec![4, 4, 5]];
        let target = vec![8, 6];
        assert_eq!(Solution::nearest_drone(drones, target), -1);
    }
}
