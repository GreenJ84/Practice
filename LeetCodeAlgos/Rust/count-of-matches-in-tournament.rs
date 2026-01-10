// You are given an integer n, the number of teams in a tournament that has strange rules:

// If the current number of teams is even, each team gets paired with another team. A total of n / 2 matches are played, and n / 2 teams advance to the next round.
// If the current number of teams is odd, one team randomly advances in the tournament, and the rest gets paired. A total of (n - 1) / 2 matches are played, and (n - 1) / 2 + 1 teams advance to the next round.
// Return the number of matches played in the tournament until a winner is decided.

struct Solution;
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut ans = 0;
        
        let mut n = n;
        while n > 1 {
            ans += n / 2;
            n = if n % 2 == 0 {
                n / 2
            } else {
                n / 2 + 1
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::number_of_matches(7), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::number_of_matches(14), 13);
    }

    #[test]
    fn test_single_team() {
        assert_eq!(Solution::number_of_matches(1), 0);
    }

    #[test]
    fn test_two_teams() {
        assert_eq!(Solution::number_of_matches(2), 1);
    }

    #[test]
    fn test_power_of_two() {
        assert_eq!(Solution::number_of_matches(8), 7);
        assert_eq!(Solution::number_of_matches(16), 15);
    }

    #[test]
    fn test_odd_numbers() {
        assert_eq!(Solution::number_of_matches(3), 2);
        assert_eq!(Solution::number_of_matches(5), 4);
    }

    #[test]
    fn test_large_input() {
        assert_eq!(Solution::number_of_matches(200), 199);
    }
}
