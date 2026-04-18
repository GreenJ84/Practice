// You are given a 0-indexed array nums of length n containing distinct positive integers. Return the minimum number of right shifts required to sort nums and -1 if this is not possible.

// A right shift is defined as shifting the element at index i to index (i + 1) % n, for all indices.

struct Solution;
impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut drops = if nums[0] < nums[n - 1] { 1 } else { 0 };
        let mut drop_index = if drops == 1 { 0 } else { n };

        for i in 1..n {
            if nums[i] < nums[i - 1] {
                if drops >= 1 {
                    return -1;
                }
                drops += 1;
                drop_index = i;
            }
        }
        return if drop_index == 0 {
            0
        } else {
            (n - drop_index) as i32
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::minimum_right_shifts(nums), 2);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 3, 5];
        assert_eq!(Solution::minimum_right_shifts(nums), 0);
    }

    #[test]
    fn test3() {
        let nums = vec![2, 1, 4];
        assert_eq!(Solution::minimum_right_shifts(nums), -1);
    }
}
