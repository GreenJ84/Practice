// You are given a positive integer n.

// A positive integer is a base-10 component if it is the product of a single digit from 1 to 9 and a non-negative power of 10. For example, 500, 30, and 7 are base-10 components, while 537, 102, and 11 are not.

// Express n as a sum of only base-10 components, using the fewest base-10 components possible.

// Return an array containing these base-10 components in descending order.

struct Solution;
impl Solution {
    pub fn decimal_representation(n: i32) -> Vec<i32> {
        let mut n = n;
        let mut ans = Vec::new();
        
        let mut level = 10;
        while n > 0 {
          let base10 = n % level;
          if base10 > 0 {
            ans.push( base10 );
          }
          n -= base10;
          level *= 10;
        }
        ans.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 537;
        let expected = vec![500, 30, 7];
        assert_eq!(Solution::decimal_representation(n), expected);
    }

    #[test]
    fn test_2() {
        let n = 102;
        let expected = vec![100, 2];
        assert_eq!(Solution::decimal_representation(n), expected);
    }

    #[test]
    fn test_3() {
        let n = 6;
        let expected = vec![6];
        assert_eq!(Solution::decimal_representation(n), expected);
    }
}
