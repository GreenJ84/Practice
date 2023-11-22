// Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].

// Return the array in the form [x1,y1,x2,y2,...,xn,yn].

struct Solution {}
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for i in 0..n {
            ans.push(nums[i as usize]);
            ans.push(nums[(i + n) as usize]);
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        assert_eq!(Solution::shuffle(vec![2,5,1,3,4,7], 3), vec![2,3,5,4,1,7] );
    }

    #[test]
    fn test_shuffle2() {
        assert_eq!(Solution::shuffle(vec![1,2,3,4,4,3,2,1], 4), vec![1,4,2,3,3,2,4,1]);
    }

    #[test]
    fn test_shuffle3() {
        assert_eq!(Solution::shuffle(vec![1,1,2,2], 2), vec![1,2,1,2]);
    }
}