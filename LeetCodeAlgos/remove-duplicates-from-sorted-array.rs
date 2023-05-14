// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
    // Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
    // Return k.
use std::collections::HashMap;
struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

        let mut curr = 0;
        let mut freq: HashMap<i32, i32>= HashMap::new();
        while curr < nums.len(){
            if !freq.contains_key(&nums[curr]){
                freq.insert(nums[curr], 1);
                curr += 1;
            } else {
                nums.remove(curr);
            }
        }
        nums.len() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums: Vec<i32> = vec![1,1,2]; 
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1,2]);
    }

    #[test]
    fn it_works2() {
        let mut nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0,1,2,3,4]);
    }
}