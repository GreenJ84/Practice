// You are given two integers, x and y, which represent your current location on a Cartesian grid: (x, y). You are also given an array points where each points[i] = [ai, bi] represents that a point exists at (ai, bi). A point is valid if it shares the same x-coordinate or the same y-coordinate as your location.

// Return the index (0-indexed) of the valid point with the smallest Manhattan distance from your current location. If there are multiple, return the valid point with the smallest index. If there are no valid points, return -1.

// The Manhattan distance between two points (x1, y1) and (x2, y2) is abs(x1 - x2) + abs(y1 - y2).

struct Solution {}
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut ans = [-1, i32::MAX];
        for (idx, point) in points.iter().enumerate() {
            if point[0] == x || point[1] == y {
                let dist = (x - point[0]).abs() + (y - point[1]).abs();
                if dist < ans[1] {
                    ans = [idx as i32, dist];
                }
            }
        }
        ans[0]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_valid_point() {
        assert_eq!(
            Solution::nearest_valid_point(3, 4, vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]),
            2
        );
    }

    #[test]
    fn test_nearest_valid_point_2() {
        assert_eq!(
            Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]),
            0
        );
    }

    #[test]
    fn test_nearest_valid_point_3() {
        assert_eq!(
            Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]),
            -1
        );
    }
}