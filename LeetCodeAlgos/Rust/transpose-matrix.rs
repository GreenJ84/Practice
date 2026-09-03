// Given a 2D integer array matrix, return the transpose of matrix.

// The transpose of a matrix is the matrix flipped over its main diagonal, switching the matrix's row and column indices.

// Constraints:
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 1000
// 1 <= m * n <= 105
// -109 <= matrix[i][j] <= 10^9

struct Solution;
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut ans = vec![vec![0; m]; n];
        for (r, row) in matrix.into_iter().enumerate() {
            for (c, val) in row.into_iter().enumerate() {
                ans[c][r] = val;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(Solution::transpose(matrix), expected);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(Solution::transpose(matrix), expected);
    }

    #[test]
    fn test_3() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
        ];
        let expected = vec![
            vec![1, 6, 11],
            vec![2, 7, 12],
            vec![3, 8, 13],
            vec![4, 9, 14],
            vec![5, 10, 15],
        ];
        assert_eq!(Solution::transpose(matrix), expected);
    }
}
