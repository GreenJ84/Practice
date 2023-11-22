// You are given an m x n matrix grid consisting of positive integers.

// Perform the following operation until grid becomes empty:

// Delete the element with the greatest value from each row. If multiple such elements exist, delete any of them.
// Add the maximum of deleted elements to the answer.
// Note that the number of columns decreases by one after each operation.

// Return the answer after performing the operations described above.


struct Solution {}
impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut sorting = true;
        let mut ans: i32 = 0;

        while grid.len() > 0 {
            let mut curr: i32 = 0;

            for row in (0..grid.len()).rev() {
                if sorting && grid[row].len() > 1{
                    grid[row].sort_unstable();
                }
                if let Some(x) = grid[row].pop(){
                    print!("max: {} {:?}", x, grid[row]);
                    if x > curr { 
                        curr = x;
                    }
                } else {
                    grid.pop();
                }
            }
            if sorting { sorting = false; }
            ans += curr;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_greatest_value() {
        assert_eq!(Solution::delete_greatest_value(vec![
            vec![1,2,4],
            vec![3,3,1]
        ]), 8);
    }

    #[test]
    fn test_delete_greatest_value_2() {
        assert_eq!(Solution::delete_greatest_value(vec![
            vec![10],
        ]), 10);
    }

    #[test]
    fn test_delete_greatest_value_3() {
        assert_eq!(Solution::delete_greatest_value(vec![
                vec![9,81],
                vec![33,17]
        ]), 98);
    }
}