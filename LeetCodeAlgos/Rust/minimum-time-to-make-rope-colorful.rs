// Alice has n balloons arranged on a rope. You are given a 0-indexed string colors where colors[i] is the color of the ith balloon.
//
// Alice wants the rope to be colorful. She does not want two consecutive balloons to be of the same color, so she asks Bob for help. Bob can remove some balloons from the rope to make it colorful. You are given a 0-indexed integer array neededTime where neededTime[i] is the time (in seconds) that Bob needs to remove the ith balloon from the rope.
//
// Return the minimum time Bob needs to make the rope colorful.
//

pub fn main(){
    println!("{:?}", Solution::min_cost("abaac".into(), [1,2,3,4,5].into()));
    println!("{:?}", Solution::min_cost("abc".into(), [1,2,3].into()));
    println!("{:?}", Solution::min_cost("aabaa".into(), [1,2,3,4,1].into()));
}

struct Solution{}
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let chars: Vec<char> = colors.chars().collect();
        let mut total_time = 0;
        let mut section_max = needed_time[0];
        let mut is_multi = false;
        for idx in 1..chars.len(){
            if chars[idx] == chars[idx-1]{
                if !is_multi { is_multi = true; }
                if needed_time[idx] > section_max {
                    total_time +=  section_max;
                    section_max = needed_time[idx];
                } else {
                    total_time += needed_time[idx];
                }
            } else {
                if is_multi { is_multi = false; }
                section_max = needed_time[idx];
            }
        }
        total_time
    }

    // pub fn min_cost(colors: String, mut needed_time: Vec<i32>) -> i32 {
    //     let mut chars: Vec<char> = colors.chars().collect();
    //     let mut time = 0;
    //     let mut idx = 1;
    //     let mut end = colors.len();
    //     while idx < end {
    //         let prev = chars.get(idx - 1).unwrap();
    //         let color = chars.get(idx).unwrap();
    //         if prev == color {
    //             if needed_time[idx - 1] > needed_time[idx] {
    //                 time += needed_time[idx];
    //                 chars.remove(idx);
    //                 needed_time.remove(idx);
    //             } else {
    //                 time += needed_time[idx - 1];
    //                 chars.remove(idx - 1);
    //                 needed_time.remove(idx - 1);
    //             }
    //             end -= 1;
    //         }
    //         idx+=1;
    //     }
    //     time
    // }
}