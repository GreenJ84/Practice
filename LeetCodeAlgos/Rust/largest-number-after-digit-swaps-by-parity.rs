// You are given a positive integer num. You may swap any two digits of num that have the same parity (i.e. both odd digits or both even digits).

// Return the largest possible value of num after any number of swaps.

// Constraints:
// 1 <= num <= 10^9

struct Solution;
impl Solution {
    pub fn largest_integer(mut num: i32) -> i32 {
        if num < 10 {
            return num;
        }
        let mut val = num;
        let mut odd = vec![];
        let mut even = vec![];
        while val > 0 {
            if val % 2 == 0 {
                even.push(val % 10);
            } else {
                odd.push(val % 10);
            }
            val /= 10;
        }
        odd.sort_unstable_by(|a, b| b.cmp(&a));
        even.sort_unstable_by(|a, b| b.cmp(&a));

        let mut place = 1;
        while num > 0 {
            if num % 2 == 0 {
                val += place * even.pop().unwrap();
            } else {
                val += place * odd.pop().unwrap();
            }
            num /= 10;
            place *= 10;
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::largest_integer(1234), 3412);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_integer(65875), 87655);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::largest_integer(1), 1);
    }
}
