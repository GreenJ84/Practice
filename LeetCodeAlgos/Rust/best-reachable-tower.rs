// You are given a 2D integer array towers, where towers[i] = [xi, yi, qi] represents the coordinates (xi, yi) and quality factor qi of the ith tower.

// You are also given an integer array center = [cx, cy​​​​​​​] representing your location, and an integer radius.

// A tower is reachable if its Manhattan distance from center is less than or equal to radius.

// Among all reachable towers:

// Return the coordinates of the tower with the maximum quality factor.
// If there is a tie, return the tower with the lexicographically smallest coordinate. If no tower is reachable, return [-1, -1].
// The Manhattan Distance between two cells (xi, yi) and (xj, yj) is |xi - xj| + |yi - yj|.
// A coordinate [xi, yi] is lexicographically smaller than [xj, yj] if xi < xj, or xi == xj and yi < yj.

// |x| denotes the absolute value of x.

// Constraints:
// 1 <= towers.length <= 10^5
// towers[i] = [xi, yi, qi]
// center = [cx, cy]
// 0 <= xi, yi, qi, cx, cy <= 10^5​​​​​​​
// 0 <= radius <= 10^5

struct Solution;
use std::cmp::Ordering;
use std::convert::TryInto;
impl Solution {
    pub fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
        let mut best_tower: Option<[i32; 3]> = None;
        for t in towers.into_iter() {
            let viable = (t[0] - center[0]).abs() + (t[1] - center[1]).abs() <= radius;
            match (viable, best_tower.as_ref()) {
                (false, _) => {}
                (true, None) => {
                    best_tower = Some(t.try_into().unwrap());
                }
                (true, Some(&[x, y, q])) => match (t[2].cmp(&q), t[0].cmp(&x), t[1].cmp(&y)) {
                    (Ordering::Greater, _, _)
                    | (Ordering::Equal, Ordering::Less, _)
                    | (Ordering::Equal, Ordering::Equal, Ordering::Less) => {
                        best_tower = Some(t.try_into().unwrap());
                    }
                    _ => {}
                },
            }
        }

        match best_tower {
            None => vec![-1, -1],
            Some(tower) => {
                vec![tower[0], tower[1]]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let towers = vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]];
        let center = vec![1, 1];
        let radius = 2;
        let result = Solution::best_tower(towers, center, radius);
        assert_eq!(result, vec![3, 1]);
    }

    #[test]
    fn test_2() {
        let towers = vec![vec![1, 3, 4], vec![2, 2, 4], vec![4, 4, 7]];
        let center = vec![0, 0];
        let radius = 5;
        let result = Solution::best_tower(towers, center, radius);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_3() {
        let towers = vec![vec![5, 6, 8], vec![0, 3, 5]];
        let center = vec![1, 2];
        let radius = 1;
        let result = Solution::best_tower(towers, center, radius);
        assert_eq!(result, vec![-1, -1]);
    }
}
