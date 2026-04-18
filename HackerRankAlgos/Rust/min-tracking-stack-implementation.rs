// Implement a stack that supports push, pop, top, and getMin operations in O(1) time, where getMin returns the minimum element.

// fn processCouponStackOperations(operations: &[String]) -> Vec<i32> {
fn process_coupon_stack_operations(operations: &[String]) -> Vec<i32> {
    let mut ans = Vec::<i32>::new();

    let mut stack = Vec::<i32>::new();
    let mut min = (i32::MAX, usize::MAX);
    for op in operations {
        match op.as_ref() {
            "top" => {
                ans.push(stack[stack.len() - 1]);
            }
            "pop" => {
                stack.pop();
                if stack.is_empty() {
                    min = (i32::MAX, usize::MAX);
                } else if stack.len() == min.1 {
                    min = (i32::MAX, usize::MAX);
                    for i in 0..stack.len() {
                        if stack[i] < min.0 {
                            min = (stack[i], i);
                        }
                    }
                }
            }
            "getMin" => {
                ans.push(min.0);
            }
            push => {
                let val = push.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
                stack.push(val);
                if val < min.0 {
                    min = (val, stack.len() - 1);
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let operations = vec![
            "push 2".to_string(),
            "push 0".to_string(),
            "push 3".to_string(),
            "push 0".to_string(),
            "getMin".to_string(),
            "pop".to_string(),
            "getMin".to_string(),
            "pop".to_string(),
            "top".to_string(),
            "getMin".to_string(),
        ];
        let expected = vec![0, 0, 0, 0];
        assert_eq!(process_coupon_stack_operations(&operations), expected);
    }

    #[test]
    fn test_2() {
        let operations = vec![
            "push 1".to_string(),
            "push 2".to_string(),
            "push 3".to_string(),
            "getMin".to_string(),
            "pop".to_string(),
            "getMin".to_string(),
            "top".to_string(),
        ];
        let expected = vec![1, 1, 2];
        assert_eq!(process_coupon_stack_operations(&operations), expected);
    }
}
