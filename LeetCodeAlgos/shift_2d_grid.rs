// Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.

// In one shift operation:

// Element at grid[i][j] moves to grid[i][j + 1].
// Element at grid[i][n - 1] moves to grid[i + 1][0].
// Element at grid[m - 1][n - 1] moves to grid[0][0].
// Return the 2D grid after applying shift operation k times.


struct Solution {}
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // Get dimentions
        let row_size = grid.len();
        let col_size = grid[0].len();
        let items = col_size * row_size;
        // Calculate wrap offset return if none
        let k = (k as usize) % items;
        if k == 0 {
            return grid;
        }
        // Flatten the grid
        let mut v = grid.concat();
        // Perform in-place shifting
        v.rotate_right(k);
        // Reconstruct the grid
        v.chunks(col_size).map(|ch| ch.to_vec()).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_grid() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
    }

    #[test]
    fn test_shift_grid_2() {
        assert_eq!(
            Solution::shift_grid(vec![vec![3,8,1,9], vec![19,7,2,5], vec![4,6,11,10], vec![12,0,21,13]], 4),
            vec![vec![12,0,21,13],vec![3,8,1,9],vec![19,7,2,5],vec![4,6,11,10]]
        );
    }

    #[test]
    fn test_shift_grid_3() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 9),
            vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]
        );
    }
}