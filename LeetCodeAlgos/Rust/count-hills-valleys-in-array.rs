// You are given a 0-indexed integer array nums. An index i is part of a hill in nums if the closest non-equal neighbors of i are smaller than nums[i]. Similarly, an index i is part of a valley in nums if the closest non-equal neighbors of i are larger than nums[i]. Adjacent indices i and j are part of the same hill or valley if nums[i] == nums[j].
// Note that for an index to be part of a hill or valley, it must have a non-equal neighbor on both the left and right of the index.
// Return the number of hills and valleys in nums.

struct Solution {}
impl Solution {
    pub fn count_hill_valley(mut nums: Vec<i32>) -> i32 {
        nums.dedup();
        let mut ans = 0;
        if nums.len() < 3 { return ans }
        for i in 1..nums.len()-1 {
            if (nums[i-1] < nums[i] && nums[i] > nums[i+1]) || (nums[i-1] > nums[i] && nums[i] < nums[i+1]) {
                ans += 1;
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_hill_valley() {
        assert_eq!(Solution::count_hill_valley(vec![2,4,1,1,6,5]), 3);
    }

    #[test]
    fn test_count_hill_valley_2() {
        assert_eq!(Solution::count_hill_valley(vec![6,6,5,5,4,1]), 0);
    }
}