// Given an integer n and an integer array rounds. We have a circular track which consists of n sectors labeled from 1 to n. A marathon will be held on this track, the marathon consists of m rounds. The ith round starts at sector rounds[i - 1] and ends at sector rounds[i]. For example, round 1 starts at sector rounds[0] and ends at sector rounds[1]

// Return an array of the most visited sectors sorted in ascending order.

// Notice that you circulate the track in ascending order of sector numbers in the counter-clockwise direction (See the first example).

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut track: HashMap<i32, i32> = HashMap::new();
        let mut prev: i32 = 0;

        for (idx, round) in rounds.iter().enumerate() {
            if idx == 0 {
                track.insert(*round, 1);
                prev = *round;
                println!("first {:?}", prev);
            } else {
                while prev != *round {
                    prev = (prev + 1) % n;
                    if prev == 0 { prev = n; }
                    
                    track.entry(prev).and_modify(|v| *v += 1).or_insert(1);
                    println!("{:?} {:?}", prev, track);
                };
            }
        }

        let max: i32 = *track.values().max().unwrap();
        println!("max {:?}, {:?}", max, track);
        let mut ans: Vec<i32> = track.iter().filter(|(_, v)| **v == max).map(|(k, _)| *k).collect::<Vec<i32>>();
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_visited() {
        assert_eq!(Solution::most_visited(4, vec![1,3,1,2]), vec![1,2]);
    }

    #[test]
    fn test_most_visited_2() {
        assert_eq!(Solution::most_visited(2, vec![2,1,2,1,2,1,2,1,2]), vec![2]);
    }

    #[test]
    fn test_most_visited_3() {
        assert_eq!(Solution::most_visited(7, vec![1,3,5,7]), vec![1,2,3,4,5,6,7]);
    }
}