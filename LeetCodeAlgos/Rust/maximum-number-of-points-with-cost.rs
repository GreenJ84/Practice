// You are given an m x n integer matrix points (0-indexed). Starting with 0 points, you want to maximize the number of points you can get from the matrix.
//
// To gain points, you must pick one cell in each row. Picking the cell at coordinates (r, c) will add points[r][c] to your score.
//
// However, you will lose points if you pick a cell too far from the cell that you picked in the previous row. For every two adjacent rows r and r + 1 (where 0 <= r < m - 1), picking cells at coordinates (r, c1) and (r + 1, c2) will subtract abs(c1 - c2) from your score.
//
// Return the maximum number of points you can achieve.
//
// abs(x) is defined as:
//
// x for x >= 0.
// -x for x < 0.

struct Solution {}
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let n = points[0].len();
        let mut prev_row = vec![0i64;n];

        for row in &points {
            let mut prefix = vec![0i64; n];
            let mut suffix = vec![0i64; n];
            let mut curr_row = vec![0i64; n];

            prefix[0] = prev_row[0];
            for col in 1..n {
                prefix[col] = prefix[col - 1].max(prev_row[col] + col as i64);
            }

            suffix[n-1] = prev_row[n-1] - (n - 1) as i64;
            for col in (0..n-1).rev(){
                suffix[col] = suffix[col + 1].max(prev_row[col] - col as i64);
            }

            for col in 0..n {
                curr_row[col] = row[col] as i64 + (prefix[col] - col as i64).max(suffix[col] + col as i64);
            }
            prev_row = curr_row;
        }
        prev_row.into_iter().max().unwrap()
    }

    // INFO Greedy Approach: Not Fully optimized
    // pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
    //     let mut ans: i64 = 0;
    //     let mut prev_col: i32 = 0;
    //
    //     for row in 0..points.len() {
    //         let mut curr_max = (0, 0);
    //         for col in 0..points[0].len() {
    //             if row == 0 && points[row][col] > curr_max.1 {
    //                 curr_max = (col as i32, points[row][col])
    //             } else if row > 0 && (points[row][col] - (prev_col - col as i32).abs() > curr_max.1)  {
    //                 curr_max = (col as i32, points[row][col] - (prev_col - col as i32).abs())
    //             }
    //         }
    //         ans += curr_max.1 as i64;
    //         prev_col = curr_max.0;
    //     }
    //     ans
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1(){
        assert_eq!(
            Solution::max_points(vec![vec![1,2,3],vec![1,5,1],vec![3,1,1]]),
            9
        );
    }

    #[test]
    pub fn test2(){
        assert_eq!(
            Solution::max_points(vec![vec![1,5],vec![2,3],vec![4,2]]),
            11
        );
    }
}