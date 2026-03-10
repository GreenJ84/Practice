// You are given two strings, coordinate1 and coordinate2, representing the coordinates of a square on an 8 x 8 chessboard.

// The coordinate will always represent a valid chessboard square. The coordinate will always have the letter first (indicating its column), and the number second (indicating its row).

struct Solution;
impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        return Self::get_color(&coordinate1) == Self::get_color(&coordinate2);
    }

    fn get_color(coord: &str) -> bool {
        let (row, col) = {
            let mut chars = coord.chars();
            let col = chars.next().unwrap() as u8 - b'a';
            let row = chars.next().unwrap() as u8 - b'1';
            (row, col)
        };
        return (row + col) % 2 == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::check_two_chessboards("a1".to_string(), "c3".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::check_two_chessboards("a1".to_string(), "h3".to_string()),
            false
        );
    }
}
