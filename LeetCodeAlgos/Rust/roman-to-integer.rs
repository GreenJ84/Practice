// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.

// Constraints:
// 1 <= s.length <= 15
// s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].

struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev = 0;

        for &b in s.as_bytes().iter().rev() {
            let value = match b {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                _ => 1000,
            };

            if value < prev {
                result -= value;
            } else {
                result += value;
            }

            prev = value;
        }

        result
    }

    pub fn roman_to_int1(s: String) -> i32 {
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();

        let mut ans = 0i32;
        let mut i = 0usize;
        while i < n {
            match s[i] {
                'I' => {
                    if (i + 1) < n {
                        match s[i + 1] {
                            'V' => {
                                ans += 4;
                            }
                            'X' => {
                                ans += 9;
                            }
                            _ => {
                                ans += 2;
                            }
                        }
                        i += 2;
                        continue;
                    }
                    ans += 1;
                }
                'V' => {
                    ans += 5;
                }
                'X' => {
                    if (i + 1) < n {
                        match s[i + 1] {
                            'L' => {
                                ans += 40;
                                i += 2;
                                continue;
                            }
                            'C' => {
                                ans += 90;
                                i += 2;
                                continue;
                            }
                            _ => {}
                        }
                    }
                    ans += 10;
                }
                'L' => {
                    ans += 50;
                }
                'C' => {
                    if (i + 1) < n {
                        match s[i + 1] {
                            'D' => {
                                ans += 400;
                                i += 2;
                                continue;
                            }
                            'M' => {
                                ans += 900;
                                i += 2;
                                continue;
                            }
                            _ => {}
                        }
                    }
                    ans += 100;
                }
                'D' => {
                    ans += 500;
                }
                _ => ans += 1000,
            }
            i += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "III".to_string();
        let result = Solution::roman_to_int(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let s = "LVIII".to_string();
        let result = Solution::roman_to_int(s);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_3() {
        let s = "MCMXCIV".to_string();
        let result = Solution::roman_to_int(s);
        assert_eq!(result, 1994);
    }

    #[test]
    fn test_4() {
        let s = "DCXXI".to_string();
        let result = Solution::roman_to_int(s);
        assert_eq!(result, 621);
    }
}
