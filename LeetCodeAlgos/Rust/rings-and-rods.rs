// There are n rings and each ring is either red, green, or blue. The rings are distributed across ten rods labeled from 0 to 9.

// You are given a string rings of length 2n that describes the n rings that are placed onto the rods. Every two characters in rings forms a color-position pair that is used to describe each ring where:

// The first character of the ith pair denotes the ith ring's color ('R', 'G', 'B').
// The second character of the ith pair denotes the rod that the ith ring is placed on ('0' to '9').
// For example, "R3G2B1" describes n == 3 rings: a red ring placed onto the rod labeled 3, a green ring placed onto the rod labeled 2, and a blue ring placed onto the rod labeled 1.

// Return the number of rods that have all three colors of rings on them.

// Constraints:

// rings.length == 2 * n
// 1 <= n <= 100
// rings[i] where i is even is either 'R', 'G', or 'B' (0-indexed).
// rings[i] where i is odd is a digit from '0' to '9' (0-indexed).

struct Solution;
impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods = [0u8; 10];
        for &[color, i] in rings.as_bytes().as_chunks().0 {
            rods[i as usize - b'0' as usize] |= match color {
                b'R' => 0b001,
                b'G' => 0b010,
                b'B' => 0b100,
                _ => 0,
            }
        }
        rods.iter().filter(|&r| *r == 0b111).count() as i32
    }

    pub fn count_points1(rings: String) -> i32 {
        if rings.len() < 6 {
            return 0;
        }

        let red = 1u8;
        let green = 1u8 << 1;
        let blue = 1u8 << 2;

        let mut chars = rings.chars().peekable();
        let mut rings = [0u8; 10];
        while chars.peek().is_some() {
            let color = chars.next().unwrap();
            let ring = chars.next().unwrap();
            let ring = &mut rings[(ring as u8 - b'0') as usize];
            match color {
                'R' => {
                    if red & *ring == 0 {
                        *ring += red;
                    }
                }
                'G' => {
                    if green & *ring == 0 {
                        *ring += green;
                    }
                }
                'B' => {
                    if blue & *ring == 0 {
                        *ring += blue;
                    }
                }
                _ => {
                    panic!("Invalid Color");
                }
            }
        }
        let valid = red + green + blue;
        rings.iter().filter(|&r| *r == valid).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let rings = "B0B6G0R6R0R6G9".to_string();
        let result = Solution::count_points(rings);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let rings = "B0R0G0R9R0B0G0".to_string();
        let result = Solution::count_points(rings);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let rings = "G4".to_string();
        let result = Solution::count_points(rings);
        assert_eq!(result, 0);
    }
}
