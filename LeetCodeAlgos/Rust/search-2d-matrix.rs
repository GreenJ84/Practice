// You are given an m x n integer matrix matrix with the following two properties:

// Each row is sorted in non-decreasing order.
// The first integer of each row is greater than the last integer of the previous row.
// Given an integer target, return true if target is in matrix or false otherwise.

// You must write a solution in O(log(m * n)) time complexity.

struct Solution {}
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix[0].len();
        for row in matrix.iter() {
            if row[0] == target || row[n - 1] == target {
                return true;
            }
            else if row[0] < target && row[n - 1] > target {
                for i in 1..n - 1 {
                    if row[i] == target {
                        return true;
                    }
                    else if row[i] < target && row[i + 1] > target {
                        break;
                    }
                }
                break;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        assert_eq!(Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3), true);
    }

    #[test]
    fn test_search_matrix_2() {
        assert_eq!(Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 13), false);
    }
}