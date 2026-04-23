// You are given a n x n 2D array grid containing distinct elements in the range [0, n2 - 1].

// Implement the NeighborSum class:

// NeighborSum(int [][]grid) initializes the object.
// int adjacentSum(int value) returns the sum of elements which are adjacent neighbors of value, that is either to the top, left, right, or bottom of value in grid.
// int diagonalSum(int value) returns the sum of elements which are diagonal neighbors of value, that is either to the top-left, top-right, bottom-left, or bottom-right of value in grid.

const ADJACENT_DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const DIAGONAL_DIRS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
struct NeighborSum {
    index: Vec<(usize, usize)>,
    grid: Vec<Vec<i32>>,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut index = vec![(0, 0); grid.len() * grid.len()];
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let value = grid[row][col] as usize;
                index[value] = (row, col);
            }
        }
        Self {
            index,
            grid,
        }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let (row, col) = self.index[value as usize];
        let mut sum = 0;

        for (dr, dc) in ADJACENT_DIRS {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            if new_row >= 0 && new_row < self.grid.len() as i32 && new_col >= 0 && new_col < self.grid[0].len() as i32 {
                sum += self.grid[new_row as usize][new_col as usize];
            }
        }
        return sum;
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let (row, col) = self.index[value as usize];
        let mut sum = 0;

        for (dr, dc) in DIAGONAL_DIRS {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            if new_row >= 0 && new_row < self.grid.len() as i32 && new_col >= 0 && new_col < self.grid[0].len() as i32 {
                sum += self.grid[new_row as usize][new_col as usize];
            }
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let neighbor_sum = NeighborSum::new(grid);
        assert_eq!(neighbor_sum.adjacent_sum(1), 6);
        assert_eq!(neighbor_sum.adjacent_sum(4), 16);
        assert_eq!(neighbor_sum.diagonal_sum(4), 16);
        assert_eq!(neighbor_sum.diagonal_sum(8), 4);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1, 2, 0, 3], vec![4, 7, 15, 6], vec![8, 9, 10, 11], vec![12, 13, 14, 5]];
        let neighbor_sum = NeighborSum::new(grid);
        assert_eq!(neighbor_sum.adjacent_sum(15), 23);
        assert_eq!(neighbor_sum.diagonal_sum(9), 45);
    }
}