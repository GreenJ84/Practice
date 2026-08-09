// Given a string s, return the maximum length of a substring such that it contains at most two occurrences of each character.

struct Solution;
impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut checks: [[Option<usize>; 2]; 26] = [[None; 2]; 26];
        let mut start = 0;

        let mut ans = 0;
        for (i, ch) in s.chars().enumerate() {
            let idx = (ch as u8 - b'a') as usize;
            let check = &mut checks[idx];
            match check {
                [Some(fir), Some(sec)] => {
                    println!("{}", i - start);
                    ans = ans.max(i - start);
                    start = start.max(*fir + 1);
                    check[0] = Some(*sec);
                    check[1] = Some(i);
                }
                [Some(_), None] => {
                    check[1] = Some(i);
                }
                _ => {
                    check[0] = Some(i);
                }
            }
        }
        ans.max(s.len() - start) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "bcbbbcba".to_string();
        assert_eq!(Solution::maximum_length_substring(s), 4);
    }

    #[test]
    fn test_2() {
        let s = "aaaa".to_string();
        assert_eq!(Solution::maximum_length_substring(s), 2);
    }

    #[test]
    fn test_3() {
        let s = "bc".to_string();
        assert_eq!(Solution::maximum_length_substring(s), 2);
    }

    #[test]
    fn test_4() {
        let s = "aaabababba".to_string();
        assert_eq!(Solution::maximum_length_substring(s), 4);
    }
}
