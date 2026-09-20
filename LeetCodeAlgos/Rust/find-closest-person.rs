// You are given three integers x, y, and z, representing the positions of three people on a number line:

// x is the position of Person 1.
// y is the position of Person 2.
// z is the position of Person 3, who does not move.
// Both Person 1 and Person 2 move toward Person 3 at the same speed.

// Determine which person reaches Person 3 first:

// Return 1 if Person 1 arrives first.
// Return 2 if Person 2 arrives first.
// Return 0 if both arrive at the same time.
// Return the result accordingly.

// Constraints:
// 1 <= x, y, z <= 100

struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match (x - z).abs().cmp(&(y - z).abs()) {
            Ordering::Greater => 2,
            Ordering::Equal => 0,
            Ordering::Less => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_closest(2, 7, 4), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_closest(2, 5, 6), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_closest(1, 5, 3), 0);
    }
}
