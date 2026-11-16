// Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and may press a key for too long, resulting in a character being typed multiple times.

// Although Alice tried to focus on her typing, she is aware that she may still have done this at most once.

// You are given a string word, which represents the final output displayed on Alice's screen.

// Return the total number of possible original strings that Alice might have intended to type.

// Constraints:
// 1 <= word.length <= 100
// word consists only of lowercase English letters.

struct Solution;
impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        if word.is_empty() {
            return 0;
        }
        let mut ans = 1i32;
        let mut last = 27u8;
        for &b in word.as_bytes().iter() {
            println!("{}", b);
            if b == last {
                ans += 1;
            }
            last = b;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "abbcccc".to_string();
        assert_eq!(Solution::possible_string_count(input), 5);
    }
    #[test]
    fn test2() {
        let input = "aaaa".to_string();
        assert_eq!(Solution::possible_string_count(input), 4);
    }
    #[test]
    fn test3() {
        let input = "abcd".to_string();
        assert_eq!(Solution::possible_string_count(input), 1);
    }
        #[test]
    fn test4() {
        let input = "zyyxxxx".to_string();
        assert_eq!(Solution::possible_string_count(input), 5);
    }
    #[test]
    fn test5() {
        let input = "zzzz".to_string();
        assert_eq!(Solution::possible_string_count(input), 4);
    }
    #[test]
    fn test6() {
        let input = "wxyz".to_string();
        assert_eq!(Solution::possible_string_count(input), 1);
    }
}

