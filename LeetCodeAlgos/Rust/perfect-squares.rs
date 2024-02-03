// Given an integer n, return the least number of perfect square numbers that sum to n.

// A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        // One square if n itself is the square
        if ((n as f64).sqrt() as i32).pow(2) == n {
            return 1;
        }
        // 4 squares if n doesnt satisfy Legendre's theorum
        let mut x = n;
        while x % 4 == 0 {
            x /= 4;
        }
        if x % 8 == 7 {
            return 4;
        }
        // 2 squares if Two nums squared add to n
        for i in 1..=(n as f64).sqrt() as i32 {
            let leftover = n - i * i;
            if ((leftover as f64).sqrt() as i32).pow(2) == leftover{
                return 2;
            }
        }
        // 3 squares if all prior fail
        return 3;
    }
}

// use std::cmp::min;
// impl Solution {
//     pub fn num_squares(n: i32) -> i32 {
//         let mut dp = vec![n; n as usize + 1];
//         dp[0] = 0;

//         for curr_step in 1..=n as usize{
//             for num in 1..=curr_step {
//                 let square = num * num;
//                 if square > curr_step as usize {
//                     break;
//                 }
//                 dp[curr_step] = min(dp[curr_step], dp[curr_step - square] + 1);
//             }
//         }

//         dp[n as usize]
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares1() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn test_num_squares2() {
        assert_eq!(Solution::num_squares(13), 2);
    }
}
