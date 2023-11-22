// Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane and an integer k, return the k closest points to the origin (0, 0).
// The distance between two points on the X-Y plane is the Euclidean distance (i.e., √(x1 - x2)2 + (y1 - y2)2).

// You may return the answer in any order. The answer is guaranteed to be unique (except for the order that it is in).

struct Solution {}
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut new_points: Vec<(i32, Vec<i32>)> = Vec::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        for point in points.iter() {
            new_points.push((point[0].pow(2) + point[1].pow(2), point.to_vec()));
        }

        new_points.sort_by(|a, b| a.0.cmp(&b.0));
        for point in new_points.iter().take(k as usize) {
            res.push(point.1.clone());
        }
        return res;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_closest() {
        assert_eq!(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
    }

    #[test]
    fn test_k_closest_2() {
        assert_eq!(
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]]
        );
    }

    #[test]
    fn test_k_closest_3() {
    }
}