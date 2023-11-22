// Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.

// Return any array that satisfies this condition.

struct Solution {}
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums.len();
        let mut curr = 0;
        while curr < n {
            println!("{}", nums[curr]);
            if nums[curr] % 2 == 1 {
                let temp = nums.remove(curr);
                nums.push(temp);
                n -= 1;
            } else {
                curr += 1;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_by_parity() {
        assert_eq!(Solution::sort_array_by_parity(vec![3,1,2,4]), vec![2,4,3,1])
    }

    #[test]
    fn test_sort_array_by_parity_2() {
        assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
    }

    #[test]
    fn test_sort_array_by_parity_3() {
        assert_eq!(Solution::sort_array_by_parity(vec![0,1,2,3,4,5,6,7,8,9]), vec![0,2,4,6,8,1,3,5,7,9]);
    }
}