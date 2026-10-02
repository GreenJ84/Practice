// You are given two integers red and blue representing the count of red and blue colored balls. You have to arrange these balls to form a triangle such that the 1st row will have 1 ball, the 2nd row will have 2 balls, the 3rd row will have 3 balls, and so on.

// All the balls in a particular row should be the same color, and adjacent rows should have different colors.

// Return the maximum height of the triangle that can be achieved.

// Constraints:
// 1 <= red, blue <= 100

struct Solution;
impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        Self::find_triangle(red, blue).max(Self::find_triangle(blue, red))
    }

    fn find_triangle(mut first: i32, mut second: i32) -> i32 {
        let mut height = 0;
        loop {
            if height % 2 == 1 {
                if first < height {
                    break height - 1;
                }
                first -= height;
            } else {
                if second < height {
                    break height - 1;
                }
                second -= height;
            }
            height += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let red = 2;
        let blue = 4;
        assert_eq!(Solution::max_height_of_triangle(red, blue), 3);
    }

    #[test]
    fn test_2() {
        let red = 2;
        let blue = 1;
        assert_eq!(Solution::max_height_of_triangle(red, blue), 2);
    }

    #[test]
    fn test_3() {
        let red = 1;
        let blue = 1;
        assert_eq!(Solution::max_height_of_triangle(red, blue), 1);
    }

    #[test]
    fn test_4() {
        let red = 10;
        let blue = 1;
        assert_eq!(Solution::max_height_of_triangle(red, blue), 2);
    }
}
