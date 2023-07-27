// You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.

// You are given an integer array nums representing the data status of this set after the error.

// Find the number that occurs twice and the number that is missing and return them in the form of an array.

struct Solution {}
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut track = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            track[nums[i] as usize] += 1;
        }

        let mut ans: Vec<i32> = Vec::from([0, 0]);
        for i in 1..track.len() {
            if track[i] == 2 {
                ans[0] = i as i32;
            }
            else if track[i] == 0 {
                ans[1] = i as i32;
            }

            if ans[0] != 0 && ans[1]!= 0 {
                return ans;
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_error_nums() {
        assert_eq!(Solution::find_error_nums(vec![1,2,2,4]), vec![2,3]);
    }

    #[test]
    fn test_find_error_nums2() {
        assert_eq!(Solution::find_error_nums(vec![1,1]), vec![1,2]);
    }

}