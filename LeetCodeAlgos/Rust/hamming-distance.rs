// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

// Given two integers x and y, return the Hamming distance between them.

struct Solution;
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let z = x ^ y;
        format!("{:b}", z).matches('1').count() as i32
    }
}