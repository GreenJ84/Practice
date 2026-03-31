// Given a string s and a character c that occurs in s, return an array of integers answer where answer.length == s.length and answer[i] is the distance from index i to the closest occurrence of character c in s.

// The distance between two indices i and j is abs(i - j), where abs is the absolute value function.

struct Solution;
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let chars = s.chars().collect::<Vec<_>>();
        let n = chars.len();

        let mut prev = n;
        let mut ans = vec![i32::MAX; s.len()];
        for i in 0..n {
          if chars[i] == c {
            prev = i;
            ans[i] = 0;
          } else if prev != n {
            ans[i] = (i - prev) as i32;
          }
        }

        for i in (0..prev).rev() {
          if chars[i] == c {
            prev = i;
          } else {
            ans[i] = ans[i].min((prev - i) as i32);
          }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1() {
        let s = "loveleetcode".to_string();
        let c = 'e';
        let expected = vec![3,2,1,0,1,0,0,1,2,2,1,0];
        assert_eq!(Solution::shortest_to_char(s, c), expected);
    }

    #[test]
    fn test_2() {
        let s = "aaab".to_string();
        let c = 'b';
        let expected = vec![3,2,1,0];
        assert_eq!(Solution::shortest_to_char(s, c), expected);
    }
}