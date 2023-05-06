// Given a string s, return the number of segments in the string.
    // A segment is defined to be a contiguous sequence of non-space characters.

struct Solution {}
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split(' ').filter(|s|!s.is_empty()).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_segments() {
        assert_eq!(Solution::count_segments("  hello  ".to_string()), 1);
    }

    #[test]
    fn test_count_segments2() {
        assert_eq!(Solution::count_segments("There is about 8 different pieces to this.".to_string()), 8);
    }

    #[test]
    fn test_count_segments3() {
        assert_eq!(Solution::count_segments("  Did    you    figure   it    out?".to_string()), 5);
    }

    #[test]
    fn test_count_segments4() {
        assert_eq!(Solution::count_segments("      ".to_string()), 0);
    }
}