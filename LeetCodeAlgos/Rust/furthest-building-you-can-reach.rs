// You are given an integer array heights representing the heights of buildings, some bricks, and some ladders.

// You start your journey from building 0 and move to the next building by possibly using bricks or ladders.

// While moving from building i to building i+1 (0-indexed),

// If the current building's height is greater than or equal to the next building's height, you do not need a ladder or bricks.
// If the current building's height is less than the next building's height, you can either use one ladder or (h[i+1] - h[i]) bricks.
// Return the furthest building index (0-indexed) you can reach if you use the given ladders and bricks optimally.

struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut diff_heap = BinaryHeap::new();

        for i in 1..heights.len() {
            let diff = heights[i] - heights[i - 1];
            if diff <= 0 {
                ans += 1;
            } else if diff <= bricks {
                diff_heap.push(diff);
                bricks -= diff;
                ans += 1;
            } else if ladders > 0 {
                if let Some(&top_diff) = diff_heap.peek() {
                    if top_diff > diff {
                        diff_heap.pop();
                        diff_heap.push(diff);
                        bricks += top_diff - diff;
                    }
                }
                ladders -= 1;
                ans += 1;
            } else { break;}
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_furthest_building() {
        assert_eq!(
            Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1),
            4,
        )
    }

    #[test]
    fn test_furthest_building2() {
        assert_eq!(
            Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
            7,
        )
    }

    #[test]
    fn test_furthest_building3() {
        assert_eq!(
            Solution::furthest_building(vec![14,3,19,3], 17, 0),
            3,
        )
    }
}
