// You are given a 0-indexed 2D integer array nums representing the coordinates of the cars parking on a number line. For any index i, nums[i] = [starti, endi] where starti is the starting point of the ith car and endi is the ending point of the ith car.

// Return the number of integer points on the line that are covered with any part of a car.

struct Solution;
impl Solution {
    pub fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
      nums.sort_unstable_by_key(|x| x[0]);
      let (mut intersect, mut last) = (0i32, -1i32);
      for car in &nums {
        if car[0] > last {
          intersect += car[1] - car[0] + 1;
          last = car[1];
        } else if car[1] > last {
          intersect += car[1] - last;
          last = car[1];
        }
      }
      intersect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![vec![3,6],vec![1,5],vec![4,7]];
        assert_eq!(Solution::number_of_points(nums), 7);
    }

    #[test]
    fn test2() {
        let nums = vec![vec![1,3],vec![5,8]];
        assert_eq!(Solution::number_of_points(nums), 7);
    }

    #[test]
    fn test3() {
        let nums = vec![vec![1,2],vec![4,5],vec![7,8]];
        assert_eq!(Solution::number_of_points(nums), 6);
    }
}

// Example 1:

// Input: nums = [[3,6],[1,5],[4,7]]
// Output: 7
// Explanation: All the points from 1 to 7 intersect at least one car, therefore the answer would be 7.
// Example 2:

// Input: nums = [[1,3],[5,8]]
// Output: 7
// Explanation: Points intersecting at least one car are 1, 2, 3, 5, 6, 7, 8. There are a total of 7 points, therefore the answer would be 7.
 