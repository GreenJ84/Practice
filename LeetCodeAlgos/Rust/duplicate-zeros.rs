// Given a fixed-length integer array arr, duplicate each occurrence of zero, shifting the remaining elements to the right.
// Note that elements beyond the length of the original array are not written. Do the above modifications to the input array in place and do not return anything.

struct Solution {}
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut idx = 1;
        while idx < arr.len() {
            if arr[idx - 1] == 0 {
                arr.insert(idx, 0);
                arr.pop();
                idx += 1;
            }
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_zeros() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1,0,0,2,3,0,0,4]);
    }

    #[test]
    fn test_duplicate_zeros_2() {
        let mut arr = vec![1,2,3];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1,2,3]);
    }

    #[test]
    fn test_duplicate_zeros_3() {
        let mut arr = vec![0,2,0,4,5];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, [0,0,2,0,0]);
    }
}