// You are given two integers m and n, representing the number of rows and columns of a grid.

// Construct any m x n grid consisting only of the characters '.' and '#', where:

// '.' represents a free cell.
// '#' represents an obstacle cell.
// A valid path is a sequence of free cells that:

// Starts at the top-left cell (0, 0).
// Ends at the bottom-right cell (m - 1, n - 1).
// Moves only:
// Right, from (i, j) to (i, j + 1), or
// Down, from (i, j) to (i + 1, j).
// Return any grid such that there is exactly one valid path from the top-left cell to the bottom-right cell.

// Constraints:
// 1 <= m, n <= 25

struct Solution;
impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut grid: Vec<String> = vec![ ".".repeat(n as usize) ];
        for _ in 1..m {
            let mut row = "#".repeat(n as usize-1);
            row.push('.');
            grid.push( row );
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1() {
        let m = 2;
        let n = 3;
        let result = Solution::create_grid(m, n);
        assert_eq!(result, vec!["...".to_string(), "##.".to_string()]);
    }

    #[test]
    fn test_2() {
        let m = 3;
        let n = 3;
        let result = Solution::create_grid(m, n);
        assert_eq!(result, vec!["...".to_string(), "##.".to_string(), "##.".to_string()]);
    }

    #[test]
    fn test_3() {
        let m = 1;
        let n = 4;
        let result = Solution::create_grid(m, n);
        assert_eq!(result, vec!["....".to_string()]);
    }
}