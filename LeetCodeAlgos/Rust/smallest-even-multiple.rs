// Given a positive integer n, return the smallest positive integer that is a multiple of both 2 and n.

struct Solution;
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 {
            n
        } else {
            n * 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 5;
        assert_eq!(Solution::smallest_even_multiple(n), 10);
    }

    #[test]
    fn test2() {
        let n = 6;
        assert_eq!(Solution::smallest_even_multiple(n), 6);
    }
}
