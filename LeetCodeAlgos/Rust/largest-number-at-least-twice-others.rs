// You are given an integer array nums where the largest integer is unique.
// Determine whether the largest element in the array is at least twice as much as every other number in the array. If it is, return the index of the largest element, or return -1 otherwise.

struct Solution {}
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return -1;
        }
        let mut mx: (usize, i32) = if nums[0] > nums[1] {
            (0, nums[0])
        } else{
            (1, nums[1])
        };
        let mut sec: (usize, i32) = if nums[0] > nums[1] {
            (1, nums[1])
        } else{
            (0, nums[0])
        };
        for i in 2..nums.len() {
            if nums[i] > mx.1 {
                sec = mx;
                mx = (i, nums[i]);
            } else if nums[i] > sec.1 {
                sec = (i, nums[i]);
            }
        }
        return if mx.1 >= 2 * sec.1 { mx.0 as i32 } else { -1 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::dominant_index(vec![3,6,1,0]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::dominant_index(vec![1,2,3,4]), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::dominant_index(vec![0,0,0,1]), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::dominant_index(vec![0,0,3,2]), -1);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::dominant_index(vec![5, 20, 64, 18, 29, 99]), -1);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::dominant_index(vec![5, 10, 15, 19, 60, 6]), 4);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::dominant_index(vec![1]), -1);
    }
}