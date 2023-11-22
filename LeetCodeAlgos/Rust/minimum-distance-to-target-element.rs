// Given an integer array nums (0-indexed) and two integers target and start, find an index i such that nums[i] == target and abs(i - start) is minimized. Note that abs(x) is the absolute value of x.
// Return abs(i - start).
// It is guaranteed that target exists in nums.

struct Solution {}
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        if nums[start as usize] == target { return 0; }
        let mut dist: i32 = 1;
        for i in 1..nums.len() {
            println!("start: {}, left: {}, right: {}, dist: {}", start, start - i as i32, start as usize + i, dist);
            if start - i as i32 >= 0 {
                println!("left-num: {}", nums[start as usize - i as usize]);
                if nums[start as usize - i] == target { 
                    break;
                }
            }
            if start as usize + i < nums.len() {
                println!("right-num: {}", nums[start as usize + i]);    
                if nums[start as usize + i] == target { 
                    break;
                }
            }
            dist+=1;
        }
        dist
    }
}

fn main(){
    println!("{}", Solution::get_min_distance(vec![1,2,3,4,5], 1, 4));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_min_distance(vec![1,2,3,4,5], 5, 3), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::get_min_distance(vec![1,1,1,1,1,1,1,1,1,1], 1, 0), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::get_min_distance(vec![1,2,3,4,5], 1, 4), 4);
    }
}