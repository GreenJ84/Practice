// You are given a 2D 0-indexed integer array dimensions.

// For all indices i, 0 <= i < dimensions.length, dimensions[i][0] represents the length and dimensions[i][1] represents the width of the rectangle i.

// Return the area of the rectangle having the longest diagonal. If there are multiple rectangles with the longest diagonal, return the area of the rectangle having the maximum area.

struct Solution;
impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let (mut max_diag, mut max_area) = (0i32, 0i32);
        for dim in &dimensions {
            let diag = dim[0].pow(2) + dim[1].pow(2);
            if diag > max_diag {
                max_diag = diag;
                max_area = dim[0] * dim[1];
            } else if diag == max_diag {
                let area = dim[0] * dim[1];
                if area > max_area {
                    max_area = area;
                }
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let dimensions = vec![vec![9, 3], vec![8, 6]];
        assert_eq!(Solution::area_of_max_diagonal(dimensions), 48);
    }

    #[test]
    fn test_2() {
        let dimensions = vec![vec![3, 4], vec![4, 3]];
        assert_eq!(Solution::area_of_max_diagonal(dimensions), 12);
    }

    #[test]
    fn test_3() {
        let dimensions = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::area_of_max_diagonal(dimensions), 9);
    }

        #[test]
    fn test_4() {
        let dimensions = vec![vec![100, 100], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::area_of_max_diagonal(dimensions), 100);
    }
}
