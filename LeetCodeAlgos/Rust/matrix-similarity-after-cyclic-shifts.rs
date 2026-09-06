// You are given an m x n integer matrix mat and an integer k. The matrix rows are 0-indexed.

// The following process happens k times:

// Even-indexed rows (0, 2, 4, ...) are cyclically shifted to the left.


// Odd-indexed rows (1, 3, 5, ...) are cyclically shifted to the right.


// Return true if the final modified matrix after k steps is identical to the original matrix, and false otherwise.

// Constraints:
// 1 <= mat.length <= 25
// 1 <= mat[i].length <= 25
// 1 <= mat[i][j] <= 25
// 1 <= k <= 50

struct Solution;
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let n = mat[0].len();
        let diff = k as usize % n;
        if diff == 0 {
            return true;
        }

        for (r, row) in mat.iter().enumerate() {
            for i in 0..n {
                let j = if r % 2 == 0 {
                    (i + diff) % n
                } else {
                    (i + n - diff) % n
                };
                if row[i] != row[j] {
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
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 4;
        assert_eq!(Solution::are_similar(mat, k), false);
    }

    #[test]
    fn test_2() {
        let mat = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
        let k = 2;
        assert_eq!(Solution::are_similar(mat, k), true);
    }

    #[test]
    fn test_3() {
        let mat = vec![vec![2, 2], vec![2, 2]];
        let k = 3;
        assert_eq!(Solution::are_similar(mat, k), true);
    }
}
