// An image smoother is a filter of the size 3 x 3 that can be applied to each cell of an image by rounding down the average of the cell and the eight surrounding cells (i.e., the average of the nine cells in the blue smoother). If one or more of the surrounding cells of a cell is not present, we do not consider it in the average (i.e., the average of the four cells in the red smoother).

// Given an m x n integer matrix img representing the grayscale of an image, return the image after applying the smoother on each cell of it.

// Constraints:
// m == img.length
// n == img[i].length
// 1 <= m, n <= 200
// 0 <= img[i][j] <= 255

struct Solution;
impl Solution {
    // Best overall Runtime/Memory Optimized Solution
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r_len = img.len();
        let c_len = img[0].len();
        let mut new = vec![vec![0; c_len]; r_len];
        for row in 0..r_len {
            for col in 0..c_len {
                let mut sum = 0;
                let mut count = 0;
                for r in row.saturating_sub(1)..=(row + 1).min(r_len - 1) {
                    for c in col.saturating_sub(1)..=(col + 1).min(c_len - 1) {
                        sum += img[r][c];
                        count += 1;
                    }
                }
                new[row][col] = sum / count;
            }
        }
        new
    }

    // Great Memory Optimized Solution; *SHIT* RUNTIME
    pub fn image_smoother1(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r_len = img.len();
        let c_len = img[0].len();
        let mut new = vec![vec![0; c_len]; r_len];
        for row in 0..=1 {
            for col in 0..=1 {
                if row < r_len && col < c_len {
                    new[0][0] += img[row][col];
                }
            }
        }

        for col in 1..c_len {
            new[0][col] = new[0][col - 1];
            for row in 0..=1 {
                if col + 1 < c_len && row < r_len {
                    new[0][col] += img[row][col + 1];
                }
                if col >= 2 && row < r_len {
                    new[0][col] -= img[row][col - 2];
                }
            }
        }

        for row in 1..r_len {
            for col in 0..c_len {
                new[row][col] = new[row - 1][col];
                for n_col in col.checked_sub(1).unwrap_or(0)..=col + 1 {
                    if n_col < c_len {
                        if row + 1 < r_len {
                            new[row][col] += img[row + 1][n_col];
                        }
                        if row >= 2 {
                            new[row][col] -= img[row - 2][n_col];
                        }
                    }
                }
            }
        }

        for row in 0..r_len {
            for col in 0..c_len {
                match (row, col) {
                    (r, c) if (r == 0 || r == r_len - 1) && (c == 0 || col == c_len - 1) => {
                        new[row][col] = (new[row][col] as f64
                            / (r_len.min(2) * c_len.min(2)) as f64)
                            .floor() as i32;
                    }
                    (r, _) if (r == 0 || r == r_len - 1) => {
                        new[row][col] = (new[row][col] as f64
                            / (r_len.min(2) * c_len.min(3)) as f64)
                            .floor() as i32;
                    }
                    (_, c) if (c == 0 || c == c_len - 1) => {
                        new[row][col] = (new[row][col] as f64
                            / (r_len.min(3) * c_len.min(2)) as f64)
                            .floor() as i32;
                    }
                    (_, _) => {
                        new[row][col] = (new[row][col] as f64 / 9.0f64).floor() as i32;
                    }
                }
            }
        }
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let img = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let expected = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::image_smoother(img), expected);
    }

    #[test]
    fn test_2() {
        let img = vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]];
        let expected = vec![
            vec![137, 141, 137],
            vec![141, 138, 141],
            vec![137, 141, 137],
        ];
        assert_eq!(Solution::image_smoother(img), expected);
    }

    #[test]
    fn test_3() {
        let img = vec![vec![6, 9, 7]];
        let expected = vec![vec![7, 7, 8]];
        assert_eq!(Solution::image_smoother(img), expected);
    }
}