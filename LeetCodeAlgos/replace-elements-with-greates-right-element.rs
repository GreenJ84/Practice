// Given an array arr, replace every element in that array with the greatest element among the elements to its right, and replace the last element with -1.
// After doing so, return the array.

struct Solution {}
impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        for i in (0..arr.len()).rev() {
            let temp = arr[i];
            if temp > max {
                arr[i] = max;
                max = temp;
            } else {
                arr[i] = max;
            }
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_elements() {
        assert_eq!(
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18,6,6,6,1,-1]
        );
    }

    #[test]
    fn test_replace_elements2() {
        assert_eq!(
            Solution::replace_elements(vec![400]),
            vec![-1]
        );
    }
}