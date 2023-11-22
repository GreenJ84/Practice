// Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.

struct Solution {}
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let bits = format!("{:b}", n).chars().collect::<Vec<char>>();
        for i in 1..bits.len() {
            if bits[i] == bits[i - 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::has_alternating_bits(1), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::has_alternating_bits(2), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::has_alternating_bits(3), false);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::has_alternating_bits(4), false);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::has_alternating_bits(5), true);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::has_alternating_bits(7), false);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::has_alternating_bits(11), false);
    }
}