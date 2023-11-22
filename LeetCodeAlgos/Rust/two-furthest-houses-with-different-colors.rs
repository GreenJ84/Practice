// There are n houses evenly lined up on the street, and each house is beautifully painted. You are given a 0-indexed integer array colors of length n, where colors[i] represents the color of the ith house.

// Return the maximum distance between two houses with different colors.

// The distance between the ith and jth houses is abs(i - j), where abs(x) is the absolute value of x.


struct Solution {}
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        if colors[0] != colors[n - 1] { return (n - 1) as i32; }
        for i in 1..n {
            if colors[0] != colors[n - 1 - i] {
                return (n - 1 - i) as i32;
            }
            else if colors[i] != colors[n - 1] {
                return ((i - (n - 1)) as i32).abs();
                // return (i.abs_diff(n - 1)) as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_distance() {
        assert_eq!(Solution::max_distance(vec![1,1,1,6,1,1,1]), 3);
    }

    #[test]
    fn test_max_distance_2() {
        assert_eq!(Solution::max_distance(vec![1,8,3,8,3]), 4);
    }

    #[test]
    fn test_max_distance_3() {
        assert_eq!(Solution::max_distance(vec![0, 1]), 1);
    }
}