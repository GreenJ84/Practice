// You are given a 2D matrix of size m x n, consisting of non-negative integers. You are also given an integer k.

// The value of coordinate (a, b) of the matrix is the XOR of all matrix[i][j] where 0 <= i <= a < m and 0 <= j <= b < n (0-indexed).

// Find the kth largest value (1-indexed) of all the coordinates of matrix.

// Constraints:
// - m == matrix.length
// - n == matrix[i].length
// - 1 <= m, n <= 1000
// - 0 <= matrix[i][j] <= 106
// - 1 <= k <= m * n

struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut heap = BinaryHeap::<i32>::new();
        let mut xor = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                xor[i][j] = matrix[i][j];
                if i > 0 {
                    xor[i][j] ^= xor[i - 1][j];
                }
                if j > 0 {
                    xor[i][j] ^= xor[i][j - 1];
                }
                if i > 0 && j > 0 {
                    xor[i][j] ^= xor[i - 1][j - 1];
                }
                heap.push(xor[i][j]);
            }
        }
        for _ in 1..k {
            heap.pop();
        }
        heap.pop().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![5, 2], vec![1, 6]];
        let k = 1;
        assert_eq!(Solution::kth_largest_value(matrix, k), 7);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![5, 2], vec![1, 6]];
        let k = 2;
        assert_eq!(Solution::kth_largest_value(matrix, k), 5);
    }

    #[test]
    fn test_3() {
        let matrix = vec![vec![5, 2], vec![1, 6]];
        let k = 3;
        assert_eq!(Solution::kth_largest_value(matrix, k), 4);
    }
}
