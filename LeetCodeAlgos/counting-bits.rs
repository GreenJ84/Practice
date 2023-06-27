// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.

struct Solution {}
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for i in 0..=n {
            ans.push(format!("{:b}", i).chars().filter(|&c| c == '1').count() as i32);
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_count_bits_2() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}