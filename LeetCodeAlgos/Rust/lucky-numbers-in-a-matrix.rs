// Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.

// A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.

// Constraints:
// m == mat.length
// n == mat[i].length
// 1 <= n, m <= 50
// 1 <= matrix[i][j] <= 105.
// All elements in the matrix are distinct.

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());

        let mut col_map = HashMap::<usize, (i32, Option<i32>)>::new();
        col_map.insert(0, (matrix[0][0], None));

        let mut min = 0usize;
        for c in 1..n {
            let val = matrix[0][c];
            // Col tracking initialization
            col_map.insert(c, (val, None));
            // Row min search
            if val < matrix[0][min] {
                min = c;
            }
        }
        col_map.get_mut(&min).unwrap().1 = Some(matrix[0][min]);

        for r in 1..m {
            for c in 0..n {
                let val = matrix[r][c];
                // Col comparisons
                let entry = col_map.get_mut(&c).unwrap();
                entry.0 = entry.0.max(val);
                if let Some(c_max) = entry.1.as_ref() {
                    if c_max < &val {
                        entry.1 = None;
                    }
                }
                // Row min search
                if c == 0 {
                    min = 0;
                } else if val < matrix[r][min] {
                    min = c;
                }
            }
            let entry = col_map.get_mut(&min).unwrap();
            if matrix[r][min] == entry.0 {
                entry.1 = Some(matrix[r][min]);
            }
        }

        col_map
            .into_values()
            .filter_map(|v| v.1)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];
        let expected = vec![15];
        assert_eq!(Solution::lucky_numbers(matrix), expected);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]];
        let expected = vec![12];
        assert_eq!(Solution::lucky_numbers(matrix), expected);
    }

    #[test]
    fn test_3() {
        let matrix = vec![vec![7, 8], vec![1, 2]];
        let expected = vec![7];
        assert_eq!(Solution::lucky_numbers(matrix), expected);
    }
}
