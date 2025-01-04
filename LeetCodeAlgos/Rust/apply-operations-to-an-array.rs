// You are given a 0-indexed array nums of size n consisting of non-negative integers.
//
// You need to apply n - 1 operations to this array where, in the ith operation (0-indexed), you will apply the following on the ith element of nums:
//
// If nums[i] == nums[i + 1], then multiply nums[i] by 2 and set nums[i + 1] to 0. Otherwise, you skip this operation.
// After performing all the operations, shift all the 0's to the end of the array.
//
// For example, the array [1,0,2,0,0,1] after shifting all its 0's to the end, is [1,2,1,0,0,0].
// Return the resulting array.
//
// Note that the operations are applied sequentially, not all at once.

struct Solution {}
impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32>{
        let n = nums.len();

        let mut ans = Vec::with_capacity(n);
        let mut zeros = 0;
        for idx in 0..n-1 {
            if nums[idx] == nums[idx + 1] {
                nums[idx] *= 2;
                nums[idx + 1] = 0;
            }
            if nums[idx] == 0 {
                zeros+=1;
            } else {
                ans.push(nums[idx]);
            }
        }
        ans.push(nums[n-1]);
        for _ in 0..zeros {
            ans.push(0);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            Solution::apply_operations(Vec::from([1,2,2,1,1,0])),
            Vec::from([1,4,2,0,0,0])
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            Solution::apply_operations(Vec::from([0,1])),
            Vec::from([1,0])
        );
    }
}