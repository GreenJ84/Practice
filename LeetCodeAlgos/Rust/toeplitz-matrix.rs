// Given an m x n matrix, return true if the matrix is Toeplitz. Otherwise, return false.

// A matrix is Toeplitz if every diagonal from top-left to bottom-right has the same elements.

struct Solution;
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for i in 0..matrix.len() - 1 {
            for j in 0..matrix[i].len() - 1 {
                if matrix[i][j] != matrix[i + 1][j + 1] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1,2,3,4],vec![5,1,2,3],vec![9,5,1,2]];
        assert_eq!(Solution::is_toeplitz_matrix(matrix), true);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1,2],vec![2,2]];
        assert_eq!(Solution::is_toeplitz_matrix(matrix), false);
    }
}

// Constraints:

// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 20
// 0 <= matrix[i][j] <= 99
