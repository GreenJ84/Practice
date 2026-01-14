// You are given an m x n matrix M initialized with all 0's and an array of operations ops, where ops[i] = [ai, bi] means M[x][y] should be incremented by one for all 0 <= x < ai and 0 <= y < bi.

// Count and return the number of maximum integers in the matrix after performing all the operations.

struct Solution;
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut max_row = m;
        let mut max_col = n;

        for op in ops {
            max_row = max_row.min(op[0]);
            max_col = max_col.min(op[1]);
        }

        max_row * max_col
    }
    
    pub fn max_count1(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![0; n as usize]; m as usize];
        for op in ops {
            for row in 0..op[0] {
                for col in 0..op[1] {
                    grid[row as usize][col as usize] += 1;
                }
            }
        }

        let mut ans = (0i32, 0i32);
        for row in grid {
            for cell in row {
                if cell > ans.0 {
                    ans = (cell, 1);
                } else if cell == ans.0 {
                    ans.1 += 1;
                }
            }
        }
        ans.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let m = 3;
        let n = 3;
        let ops = vec![vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::max_count(m, n, ops), 4);
    }

    #[test]
    fn test_example_2() {
        let m = 3;
        let n = 3;
        let ops = vec![
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
        ];
        assert_eq!(Solution::max_count(m, n, ops), 4);
    }

    #[test]
    fn test_example_3_empty_ops() {
        let m = 3;
        let n = 3;
        let ops: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::max_count(m, n, ops), 9);
    }

    #[test]
    fn test_single_operation() {
        let m = 4;
        let n = 5;
        let ops = vec![vec![2, 3]];
        assert_eq!(Solution::max_count(m, n, ops), 6);
    }

    #[test]
    fn test_full_matrix_operation() {
        let m = 2;
        let n = 2;
        let ops = vec![vec![2, 2]];
        assert_eq!(Solution::max_count(m, n, ops), 4);
    }

    #[test]
    fn test_minimum_constraints() {
        let m = 1;
        let n = 1;
        let ops = vec![vec![1, 1]];
        assert_eq!(Solution::max_count(m, n, ops), 1);
    }

    #[test]
    fn test_large_matrix_small_ops() {
        let m = 40000;
        let n = 40000;
        let ops = vec![vec![100, 100]];
        assert_eq!(Solution::max_count(m, n, ops), 10000);
    }

    #[test]
    fn test_descending_operations() {
        let m = 5;
        let n = 5;
        let ops = vec![vec![5, 5], vec![3, 3], vec![2, 2]];
        assert_eq!(Solution::max_count(m, n, ops), 4);
    }
}
