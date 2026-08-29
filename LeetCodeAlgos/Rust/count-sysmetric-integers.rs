// You are given two positive integers low and high.

// An integer x consisting of 2 * n digits is symmetric if the sum of the first n digits of x is equal to the sum of the last n digits of x. Numbers with an odd number of digits are never symmetric.

// Return the number of symmetric integers in the range [low, high].

// Constraints:
// 1 <= low <= high <= 10^4

struct Solution;
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high)
            .filter(|&num| match num {
                n if n > 10 && n < 100 => {
                    if n % 10 == n / 10 {
                        true
                    } else {
                        false
                    }
                }
                n if n > 1000 && n < 10000 => {
                    let first = n % 10 + n / 10 % 10;
                    if first == (n / 100 % 10 + n / 1000 % 10) {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let low = 1;
        let high = 100;
        let result = Solution::count_symmetric_integers(low, high);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_2() {
        let low = 1200;
        let high = 1230;
        let result = Solution::count_symmetric_integers(low, high);
        assert_eq!(result, 4);
    }
}
