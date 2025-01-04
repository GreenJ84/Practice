// There is a biker going on a road trip. The road trip consists of n + 1 points at different altitudes. The biker starts his trip on point 0 with altitude equal 0.
//
// You are given an integer array gain of length n where gain[i] is the net gain in altitude between points i​​​​​​ and i + 1 for all (0 <= i < n). Return the highest altitude of a point.

struct Solution {}
impl Solution {
    // Memory Optimization
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (mut highest, mut curr) = (0, 0);
        for val in &gain {
            curr += val;
            highest = highest.max(curr);
        }
        highest
    }

    // pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    //     let mut highest = 0;
    //     let mut curr = 0;
    //     for idx in 0..gain.len() {
    //         curr += gain[idx];
    //         highest = highest.max(curr);
    //     }
    //     highest
    // }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn test1(){
        assert_eq!(
            Solution::largest_altitude(Vec::from([-5,1,5,0,-7])),
            1
        );
    }

    #[test]
    pub fn test2(){
        assert_eq!(
            Solution::largest_altitude(Vec::from([-4,-3,-2,-1,4,3,2])),
            0
        );
    }
}