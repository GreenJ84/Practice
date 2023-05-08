// Given a square matrix mat, return the sum of the matrix diagonals.

// Only include the sum of all the elements on the primary diagonal and all the elements on the secondary diagonal that are not part of the primary diagonal.

struct Solution {}
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut sum = if n % 2 == 0 { 0 } else { -1 * mat[n/2][n/2] };

        for i in 0..n {
            sum += mat[i][i];
            sum += mat[i][n-i-1];
        }
        sum
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_sum() {
        assert_eq!(Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 25);
    }

    #[test]
    fn test_diagonal_sum2() {
        assert_eq!(Solution::diagonal_sum(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]]), 8);
    }

    #[test]
    fn test_diagonal_sum3() {
        assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
    }
}