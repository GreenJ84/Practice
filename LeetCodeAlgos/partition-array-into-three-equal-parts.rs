// Given an array of integers arr, return true if we can partition the array into three non-empty parts with equal sums.
// Formally, we can partition the array if we can find indexes i + 1 < j with (arr[0] + arr[1] + ... + arr[i] == arr[i + 1] + arr[i + 2] + ... + arr[j - 1] == arr[j] + arr[j + 1] + ... + arr[arr.length - 1])

struct Solution {}

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        // Get Array sum
        let mut total = arr.iter().sum::<i32>();

        // If we cant make 3 equal sections return
        if total % 3 != 0{
            return false
        }

        // A target sum for a section
        let target = total / 3;
        // A section counter
        let mut segs = 3;
        // A running section Sum
        let mut curr = 0;
        // Iterate through the array
        for i in 0..arr.len() {
            // Add to running sum and remove from total sum
            curr += arr[i];
            total -= arr[i];
            // If a valid section, remove from needed and reset
            if curr == target {
                segs -= 1;
                curr = 0;
            }
            // If we have enough sections make. sure no extra sum
            if segs == 0 {
                return if total == 0 {true} else {false};
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_three_parts_equal_sum1() {
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![0,2,1,-6,6,-7,9,1,2,0,1]), 
            true
        );
    }

    #[test]
    fn test_can_three_parts_equal_sum2() {
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![0,2,1,-6,6,7,9,-1,2,0,1]), 
            false
        );
    }

    #[test]
    fn test_can_three_parts_equal_sum3() {
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![3,3,6,5,-2,2,5,1,-9,4]),
            true
        );
    }

    #[test]
    fn test_can_three_parts_equal_sum4() {
        assert_eq!(
            Solution::can_three_parts_equal_sum(vec![0,0,0,0]),
            true
        );
    }
}