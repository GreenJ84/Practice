// You are given an array of positive integers nums.

// Alice and Bob are playing a game. In the game, Alice can choose either all single-digit numbers or all double-digit numbers from nums, and the rest of the numbers are given to Bob. Alice wins if the sum of her numbers is strictly greater than the sum of Bob's numbers.

// Return true if Alice can win this game, otherwise, return false.

// Constraints:
// - 1 <= nums.length <= 100
// - 1 <= nums[i] <= 99

struct Solution;
impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let sums = nums.iter().fold((0i32, 0i32, 0i32), |mut acc, &num| {
            acc.0 += num;
            match num {
                n if n < 10 => {
                    acc.1 += n;
                }
                n if n < 100 => {
                    acc.2 += n;
                }
                _ => {}
            }
            acc
        });
        sums.1 > (sums.0 - sums.1) || sums.2 > (sums.0 - sums.2)
    }

    pub fn _can_alice_win1(nums: Vec<i32>) -> bool {
        let sums = nums.iter().fold((0i32, 0i32, 0i32), |acc, &num| match num {
            n if n < 10 => (acc.0 + n, acc.1 + n, acc.2),
            n if n < 100 => (acc.0 + n, acc.1, acc.2 + n),
            n => (acc.0 + n, acc.1, acc.2),
        });
        sums.1 > (sums.0 - sums.1) || sums.2 > (sums.0 - sums.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4, 10];
        assert_eq!(Solution::can_alice_win(nums), false);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4, 5, 14];
        assert_eq!(Solution::can_alice_win(nums), true);
    }

    #[test]
    fn test3() {
        let nums = vec![5, 5, 5, 25];
        assert_eq!(Solution::can_alice_win(nums), true);
    }
}

