// There is a programming language with only four operations and one variable X:

// ++X and X++ increments the value of the variable X by 1.
// --X and X-- decrements the value of the variable X by 1.
// Initially, the value of X is 0.

// Given an array of strings operations containing a list of operations, return the final value of X after performing all the operations.

struct Solution {}
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut ans: i32 = 0;
        for op in operations.iter() {
            match op.as_str() {
                "++X" => ans += 1,
                "X++" => ans += 1,
                "--X" => ans -= 1,
                "X--" => ans -= 1,
                _ => {}
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_value_after_operations() {
        assert_eq!(Solution::final_value_after_operations(vec!["--X".to_string(),"X++".to_string(), "X++".to_string()]), 1);
    }

    #[test]
    fn test_final_value_after_operations_2() {
        assert_eq!(Solution::final_value_after_operations(vec!["++X".to_string(),"++X".to_string(),"X++".to_string()]), 3);
    }

    #[test]
    fn test_final_value_after_operations_3() {
        assert_eq!(Solution::final_value_after_operations(vec!["++X".to_string(),"X++".to_string(),"--X".to_string(),"X--".to_string()]), 0);
    }
}