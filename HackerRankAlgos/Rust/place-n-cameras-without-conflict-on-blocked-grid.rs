// Given an NxN grid where 0 is empty and 1 is blocked, return true if N cameras can be placed on empty cells such that no two share the same row, column, or diagonal.

// Constraints

// 1 <= N <= 15
// grid.length == N
// For all 0 <= i < N, grid[i].length == N
// For all 0 <= i, j < N, grid[i][j] ∈ {0, 1}

// fn canPlaceSecurityCameras(N: i32, grid: &[Vec<i32>]) -> bool {
fn can_place_security_cameras(n: i32, grid: &[Vec<i32>]) -> bool {
    let mut available: Vec<(usize, Vec<usize>)> = grid
        .iter()
        .enumerate()
        .map(|(row, cols)| {
            let open_cols = cols
                .iter()
                .enumerate()
                .filter_map(|(col, val)| if *val == 0 { Some(col) } else { None })
                .collect();
            (row, open_cols)
        })
        .collect();

    available.sort_unstable_by_key(|(_, open_cols)| open_cols.len());
    println!("{:?}", available);

    let n = n as usize;
    let mut occupied_cols = vec![false; n];
    let mut occupied_d1 = vec![false; 2 * n + 1];
    let mut occupied_d2 = vec![false; 2 * n + 1];

    fn backtrack(
      n: usize,
      idx: usize,
      available: &[(usize, Vec<usize>)],
      occupied_cols: &mut Vec<bool>,
      occupied_d1: &mut Vec<bool>,
      occupied_d2: &mut Vec<bool>,
    ) -> bool {
      if idx == n {
        return true;
      }

      let (row, cols) = &available[idx];

      for &col in cols {
        let d1 = n - 1 + row - col;
        let d2 = row + col;

        if !occupied_cols[col] && !occupied_d1[d1] && !occupied_d2[d2] {
          occupied_cols[col] = true;
          occupied_d1[d1] = true;
          occupied_d2[d2] = true;
          if backtrack(n, idx + 1, available, occupied_cols, occupied_d1, occupied_d2){
            return true;
          }

          occupied_cols[col] = false;
          occupied_d1[d1] = false;
          occupied_d2[d2] = false;
        }
      }
      false
    }
    backtrack(n, 0, &available, &mut occupied_cols, &mut occupied_d1, &mut occupied_d2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(can_place_security_cameras(n, &grid), true);
    }

    #[test]
    fn test_2() {
        let n = 4;
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
            vec![0, 0, 1, 0],
        ];
        assert_eq!(can_place_security_cameras(n, &grid), true);
    }

    #[test]
    fn test_3() {
        let n = 4;
        let grid = vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 1],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(can_place_security_cameras(n, &grid), true);
    }
}
