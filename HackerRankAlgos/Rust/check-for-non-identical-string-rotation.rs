// Given two strings s1 and s2, return 1 if s2 is a rotation of s1 but not identical to s1, otherwise return 0.

// fn isNonTrivialRotation(s1: &str, s2: &str) -> bool {
fn is_non_trivial_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() || s1 == s2 {
        false
    } else {
        let double_s1 = format!("{}{}", s1, s1);
        if double_s1.contains(s2) {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s1 = "abcde";
        let s2 = "cdeab";
        let result = is_non_trivial_rotation(s1, s2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s1 = "abcde";
        let s2 = "abced";
        let result = is_non_trivial_rotation(s1, s2);
        assert_eq!(result, false);
    }
}

// Example

// Input:

// s1 = abcde
// s2 = cdeab
// Output:

// True
// Explanation:

// - s2 ('cdeab') is a non-trivial rotation of s1 ('abcde').
// - If you rotate 'abcde' left by 2 positions, you get 'cdeab'.
// - Since s2 is not equal to s1 an
