// You are given a positive integer n.

// Return the integer obtained by removing all zeros from the decimal representation of n.

struct Solution;
impl Solution {
    pub fn remove_zeros(n: i64) -> i64 {
        let mut stack = Vec::new();
        let mut n = n;
        while n > 0 {
            let digit = n % 10;
            if digit != 0 {
                stack.push(digit);
            }
            n /= 10;
        }

        while !stack.is_empty() {
            if n > 0 {
                n *= 10;
            }
            n += stack.pop().unwrap();
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::remove_zeros(1020030), 123);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::remove_zeros(1), 1);
    }
}
