// Alice and Bob take turns playing a game, with Alice starting first.

// Initially, there is a number n on the chalkboard. On each player's turn, that player makes a move consisting of:

// Choosing any integer x with 0 < x < n and n % x == 0.
// Replacing the number n on the chalkboard with n - x.
// Also, if a player cannot make a move, they lose the game.

// Return true if and only if Alice wins the game, assuming both players play optimally.

// Constraints:
// 1 <= n <= 1000

struct Solution;
impl Solution {
    pub fn divisor_game(mut n: i32) -> bool {
        n % 2 == 0
    }

    pub fn divisor_game1(mut n: i32) -> bool {
        let mut turns = 0;
        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
                turns += 1;
                continue;
            } else if n == 3 {
                turns += 2;
                break;
            }
            for k in (1..n / 2).rev() {
                if n % k == 0 {
                    n -= k;
                    turns += 1;
                    break;
                }
            }
        }
        turns % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let result = Solution::divisor_game(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let n = 3;
        let result = Solution::divisor_game(n);
        assert_eq!(result, false);
    }
}
