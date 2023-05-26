// Given a string columnTitle that represents the column title as appears in an Excel sheet, return its corresponding column number.

struct Solution {}
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut column_number = 0;
        for (idx, val) in column_title.chars().rev().enumerate() {
            let current_char: i32 = val as i32 - 'A' as i32 + 1;
            column_number += 26_i32.pow(idx as u32) * current_char;
        }
        column_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_number() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
    }

    #[test]
    fn test_title_to_number2() {
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    }

    #[test]
    fn test_title_to_number3() {
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }

    #[test]
    fn test_title_to_number4() {
        assert_eq!(Solution::title_to_number("FXSHRXW".to_string()), 2147483647);
    }
}