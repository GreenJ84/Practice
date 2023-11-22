// Given an integer array nums and an integer k, return the kth largest element in the array.
// Note that it is the kth largest element in the sorted order, not the kth distinct element.
// Can you solve it without sorting?

struct Solution {}
// TC: O(n) + k, SC: 0(n). Non Vec Consuming
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut copy = BinaryHeap::from(nums);
        return loop {
            k -= 1;
            if k == 0 {
                break copy.pop().unwrap();
            }
            copy.pop().unwrap();
        }
    }
}
// TC: 0(n), SC: 0. Vec Consuming
// impl Solution {
//     pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
//         nums.sort_unstable_by(|a, b| b.cmp(a));
//         nums[k as usize - 1]
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(Solution::find_kth_largest(vec![3,2,1,5,6,4], 2), 5);
    }

    #[test]
    fn test_find_kth_largest2() {
        assert_eq!(Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4), 4);
    }
}