// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

// Find two lines that together with the x-axis form a container, such that the container contains the most water.

// Return the maximum amount of water a container can store.

// Notice that you may not slant the container.

struct Solution {}
impl Solution {
    // Optimal two-pointer solution
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut max_water = 0;
        while left < right {
            let dist = (right - left) as i32;

            if height[left] < height[right] {
                max_water = max_water.max(
                    dist * height[left]
                );
                left += 1;
            } else {
                max_water = max_water.max(
                    dist * height[right]
                );
                right -= 1;
            }
        }
        max_water
    }

    // Brute force solution
    // pub fn max_area(height: Vec<i32>) -> i32 {
    //     let n = height.len();
    //     let mut max_water = 0;
    //     for left in 0..n - 1{
    //         for right in (left + 1)..n {
    //             let water = (right - left) as i32 * (height[left].min(height[right]));
    //             if water > max_water { 
    //                 max_water = water;
    //             }
    //         }
    //     }
    //     max_water
    // }
}