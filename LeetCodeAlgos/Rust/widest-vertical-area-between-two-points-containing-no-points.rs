// Given n points on a 2D plane where points[i] = [xi, yi], Return the widest vertical area between two points such that no points are inside the area.

// A vertical area is an area of fixed-width extending infinitely along the y-axis (i.e., infinite height). The widest vertical area is the one with the maximum width.

// Note that points on the edge of a vertical area are not considered included in the area.

struct Solution;
impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|i| i[0]);
        let mut ans = 0;
        for i in 1..points.len() {
          ans = ans.max(points[i][0] - points[i - 1][0]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let points = vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]];
        assert_eq!(Solution::max_width_of_vertical_area(points), 1);
    }

    #[test]
    fn test_2() {
        let points = vec![vec![3,1],vec![9,0],vec![1,0],vec![1,4],vec![5,3],vec![8,8]];
        assert_eq!(Solution::max_width_of_vertical_area(points), 3);
    }
}