// Given an integer array sorted in non-decreasing order, there is exactly one integer in the array that occurs more than 25% of the time, return that integer.

use std::collections::HashMap;


struct Solution {}
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return arr[0];
        }

        let target = (arr.len() as f64) * 0.25;
        let mut track = HashMap::new();
        for i in arr.iter(){
            println!("{} {:?} {}", i, track.get(i), target);
            if let Some(v) = track.get_mut(&i){
                *v += 1;
                if *v as f64 > target{
                    return *i;
                }
            } else {
                track.insert(i, 1);
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_special_integer(vec![1,2,2,6,6,6,6,7,10]), 6);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::find_special_integer(vec![1,1]), 1);
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::find_special_integer(vec![1]), 1);
    }

    #[test]
    fn it_works4() {
        assert_eq!(Solution::find_special_integer(vec![1,2,3,3]), 3);
    }
}