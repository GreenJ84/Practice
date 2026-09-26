// You are given an integer array nums that represents a circular array. Your task is to create a new array result of the same size, following these rules:

// For each index i (where 0 <= i < nums.length), perform the following independent actions:
// If nums[i] > 0: Start at index i and move nums[i] steps to the right in the circular array. Set result[i] to the value at the index where you land.
// If nums[i] < 0: Start at index i and move abs(nums[i]) steps to the left in the circular array. Set result[i] to the value at the index where you land.
// If nums[i] == 0: Set result[i] to nums[i].
// Return the new array result.

// Note: Since nums is circular, moving past the last element wraps around to the beginning, and moving before the first element wraps back to the end.

// Constraints:
// 1 <= nums.length <= 100
// -100 <= nums[i] <= 100

struct Solution;
impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        (0..n)
            .map(|i| nums[(i + nums[i as usize]).rem_euclid(n) as usize])
            .collect()
    }

    pub fn construct_transformed_array1(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.iter()
            .enumerate()
            .map(|(i, v)| match v {
                0 => 0,
                x if x > &0 => {
                    let new = (i + *v as usize) % n;
                    nums[new]
                }
                _ => {
                    let new = (n + i - (-*v as usize % n)) % n;
                    nums[new]
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, -2, 1, 1];
        assert_eq!(
            Solution::construct_transformed_array(nums),
            vec![1, 1, 1, 3]
        );
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 4, -1];
        assert_eq!(Solution::construct_transformed_array(nums), vec![-1, -1, 4]);
    }
}
