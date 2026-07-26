// Given an m x n binary matrix mat, return the number of special positions in mat.

// A position (i, j) is called special if mat[i][j] == 1 and all other elements in row i and column j are 0 (rows and columns are 0-indexed).

// Constraints:

// m == mat.length
// n == mat[i].length
// 1 <= m, n <= 100
// mat[i][j] is either 0 or 1.

struct Solution;
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat[0].len();
        let mut ans = 0i32;

        // Track the number of 1s in each column and whether a special position has been found in that column
        let mut cols = vec![vec![0i32, 0i32]; n];
        // Iter every row
        for row in &mat {
            // Track the ones in the row and their col placement
            let (mut row_ones, mut sp_idx) = (0i32, 0usize);
            // Iter every col in row
            for col in 0..n {
                if row[col] == 1 {
                    row_ones += 1;
                    cols[col][0] += 1;
                    if cols[col][1] == 1 {
                        ans -= 1;
                        cols[col][1] = 0;
                    }
                    sp_idx = col;
                }
            }
            // If only a single one in row, check its column
            if row_ones == 1 && cols[sp_idx][0] == 1 {
                ans += 1;
                cols[sp_idx][1] = 1;
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
        let mat = vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]];
        assert_eq!(Solution::num_special(mat), 1);
    }

    #[test]
    fn test_2() {
        let mat = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::num_special(mat), 3);
    }
}
