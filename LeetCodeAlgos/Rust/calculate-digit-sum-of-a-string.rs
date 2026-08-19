// You are given a string s consisting of digits and an integer k.

// A round can be completed if the length of s is greater than k. In one round, do the following:

// Divide s into consecutive groups of size k such that the first k characters are in the first group, the next k characters are in the second group, and so on. Note that the size of the last group can be smaller than k.
// Replace each group of s with a string representing the sum of all its digits. For example, "346" is replaced with "13" because 3 + 4 + 6 = 13.
// Merge consecutive groups together to form a new string. If the length of the string is greater than k, repeat from step 1.
// Return s after all rounds have been completed.

// Constraints:
// 1 <= s.length <= 100
// 2 <= k <= 100
// s consists of digits only.

struct Solution;
impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;
        while s.len() > k {
            let mut temp = String::new();
            let bytes = s.as_bytes();
            for ch in bytes.chunks(k) {
                let sum: i32 = ch.iter().map(|&d| (d as u8 - b'0') as i32).sum();
                temp.push_str(&sum.to_string());
            }
            s = temp;
        }
        s
    }

    pub fn digit_sum1(s: String, k: i32) -> String {
        let mut ans = s;
        while ans.len() > k as usize {
            let mut temp = String::new();
            let mut step = 0;
            let mut sum = 0;
            for ch in ans.chars() {
                sum += (ch as u8 - b'0') as i32;
                step += 1;
                if step == k {
                    println!("sum: {sum}");
                    temp.push_str(&format!("{sum}"));
                    step = 0;
                    sum = 0;
                }
            }
            if step != 0 {
                temp.push_str(&format!("{sum}"));
            }
            println!("temp: {temp}");
            ans = temp;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "11111222223".to_string();
        let k = 3;
        let result = Solution::digit_sum(s, k);
        assert_eq!(result, "135");
    }

    #[test]
    fn test_2() {
        let s = "00000000".to_string();
        let k = 3;
        let result = Solution::digit_sum(s, k);
        assert_eq!(result, "000");
    }

    #[test]
    fn test_3() {
        let s = "1234".to_string();
        let k = 2;
        let result = Solution::digit_sum(s, k);
        assert_eq!(result, "37");
    }

    #[test]
    fn test_4() {
        let s = "71818186138735364590516322993378229838446988388364431324753408563431136824898916288399".to_string();
        let k = 85;
        let result = Solution::digit_sum(s, k);
        assert_eq!(result, "4169");
    }
}
