// Given an alphanumeric string s, return the second largest numerical digit that appears in s, or -1 if it does not exist.
// An alphanumeric string is a string consisting of lowercase English letters and digits.

struct Solution {}
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut first: i32 = -1;
        let mut sec: i32 = -1;
        for ch in s.chars().filter(|c| c.is_digit(10)).map(|c| c as u32 - 48).collect::<Vec<u32>>().iter(){
            print!("{} ", ch);
            if *ch as i32 > first {
                sec = first;
                first = *ch as i32;
            } else if ((*ch as i32) < first) && ((*ch as i32) > sec) {
                sec = *ch as i32;
            }
        }
        sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::second_highest("dfa12321afd".to_string()), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::second_highest("abc1111".to_string()), -1);
    }
}