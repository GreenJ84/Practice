// Given an integer array arr, return the mean of the remaining integers after removing the smallest 5% and the largest 5% of the elements.

// Answers within 10-5 of the actual answer will be considered accepted.

struct Solution {}
impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort_unstable();

        let five_percent = (arr.len() as f64 / 100.0 * 5.0) as usize;
        let mut sum = 0.0;

        for i in five_percent..(arr.len() - five_percent) {
            sum += arr[i] as f64;
        }
        sum / (arr.len() - (2 * five_percent)) as f64
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_mean1() {
        assert_eq!(Solution::trim_mean(vec![1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3]), 2.00000);
    }

    #[test]
    fn test_trim_mean2() {
        assert_eq!(Solution::trim_mean(vec![6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]), 4.00000);
    }

    #[test]
    fn test_trim_mean3() {
        assert_eq!(Solution::trim_mean(vec![6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]), 4.77778);
    }
}