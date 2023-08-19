// Given an array of integers arr, return true if and only if it is a valid mountain array.

// Recall that arr is a mountain array if and only if:

// arr.length >= 3
// There exists some i with 0 < i < arr.length - 1 such that:
// arr[0] < arr[1] < ... < arr[i - 1] < arr[i] 
// arr[i] > arr[i + 1] > ... > arr[arr.length - 1]


struct Solution {}
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        // Nothing too short
        if arr.len() < 3 {
            return false;
        }
        // Keep a track for the peak
        let mut i = 1;
        while i < arr.len(){
            // Any equals fails
            if arr[i-1] == arr[i]{
                return false;
            }
            // Break on the first peak, should be the only
            if arr[i-1] > arr[i] {
                break;
            }
            i += 1;
        }
        // If it peaks at the last spot, a fail
        if i == arr.len() || i == 1 {
            return false;
        }
        // Check the rest is decreasing
        for j in i..arr.len(){
            if arr[j-1] <= arr[j]{
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mountain_array() {
        assert_eq!(Solution::valid_mountain_array(vec![2,1]), false);
    }

    #[test]
    fn test_valid_mountain_array_2() {
        assert_eq!(Solution::valid_mountain_array(vec![3,5,5]), false);
    }

    #[test]
    fn test_valid_mountain_array_3() {
        assert_eq!(Solution::valid_mountain_array(vec![0,3,2,1]), true);
    }

    #[test]
    fn test_valid_mountain_array_4() {
        assert_eq!(Solution::valid_mountain_array(vec![9,8,7,6,5,4,3,2,1,0]), false);
    }

    #[test]
    fn test_valid_mountain_array_5() {
        assert_eq!(Solution::valid_mountain_array(vec![0,1,2,3,4,5,6,7,8,9]), false);
    }
}