// A bit flip of a number x is choosing a bit in the binary representation of x and flipping it from either 0 to 1 or 1 to 0.

// For example, for x = 7, the binary representation is 111 and we may choose any bit (including any leading zeros not shown) and flip it. We can flip the first bit from the right to get 110, flip the second bit from the right to get 101, flip the fifth bit from the right (a leading zero) to get 10111, etc.
// Given two integers start and goal, return the minimum number of bit flips to convert start to goal.

// Constraints:
// 0 <= start, goal <= 10^9

struct Solution;
impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut check = start ^ goal;
        let mut ans = 0;
        while check > 0 {
          ans += check % 2;
          check = check >> 1;
        }
        ans
    }

    pub fn min_bit_flips2(start: i32, goal: i32) -> i32 {
        format!("{:b}", start ^ goal)
            .chars()
            .filter(|ch| ch == &'1')
            .count() as i32
    }

    pub fn min_bit_flips1(start: i32, goal: i32) -> i32 {
        let start: Vec<char> = format!("{:0>32b}", start).chars().collect();
        format!("{:0>32b}", goal)
            .chars()
            .enumerate()
            .filter(|(idx, ch)| &start[*idx] != ch)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let start = 10;
        let goal = 7;
        let result = Solution::min_bit_flips(start, goal);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let start = 3;
        let goal = 4;
        let result = Solution::min_bit_flips(start, goal);
        assert_eq!(result, 3);
    }
}
