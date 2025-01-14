// You are given two 0-indexed integer permutations A and B of length n.
//
// A prefix common array of A and B is an array C such that C[i] is equal to the count of numbers that are present at or before the index i in both A and B.
//
// Return the prefix common array of A and B.
//
// A sequence of n integers is called a permutation if it contains all integers from 1 to n exactly once.


struct Solution {}
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut seen: Vec<i32> = vec![0; n];
        let mut ans: Vec<i32> = vec![0; n];
        let mut common = 0;

        for idx in 0..n {
            if seen[(a[idx] - 1) as usize] == 1 {
                common += 1;
            }
            seen[(a[idx] - 1) as usize] += 1;
            if seen[(b[idx] - 1) as usize] == 1 {
                common += 1;
            }
            seen[(b[idx] - 1) as usize] += 1;
            ans[idx] = common;
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
            Solution::find_the_prefix_common_array(
                vec![1,3,2,4],
                vec![3,1,2,4]
            ),
            vec![0,2,3,4]
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            Solution::find_the_prefix_common_array(
                vec![2,3,1],
                vec![3,1,2]
            ),
            vec![0,1,3]
        );
    }
}