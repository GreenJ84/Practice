// Given an array of distinct integers arr, find all pairs of elements with the minimum absolute difference of any two elements.

// Return a list of pairs in ascending order(with respect to pairs), each pair [a, b] follows

// a, b are from arr
// a < b
// b - a equals to the minimum absolute difference of any two elements in arr

struct Solution;
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort_unstable();

        let mut ans = Vec::new();
        let mut min_diff = i32::MAX;
        for i in 1..sorted_arr.len() {
            let diff = sorted_arr[i] - sorted_arr[i - 1];
            if diff < min_diff {
                min_diff = diff;
                ans.clear();
                ans.push(vec![sorted_arr[i - 1], sorted_arr[i]]);
            } else if diff == min_diff {
                ans.push(vec![sorted_arr[i - 1], sorted_arr[i]]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let arr = vec![4, 2, 1, 3];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
    }

    #[test]
    fn test_example_2() {
        let arr = vec![1, 3, 6, 10, 15];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![1, 3]]);
    }

    #[test]
    fn test_example_3() {
        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);
    }

    #[test]
    fn test_two_elements() {
        let arr = vec![1, 2];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![1, 2]]);
    }

    #[test]
    fn test_negative_numbers() {
        let arr = vec![-5, -3, -1, 0, 2];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(
            result,
            vec![vec![-1, 0]]
        );
    }

    #[test]
    fn test_large_gap() {
        let arr = vec![1, 1000000, -1000000];
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result, vec![vec![1, 1000000]]);
    }
}
