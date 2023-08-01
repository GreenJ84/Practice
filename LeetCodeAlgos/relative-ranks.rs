// You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.

// The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:

// The 1st place athlete's rank is "Gold Medal".
// The 2nd place athlete's rank is "Silver Medal".
// The 3rd place athlete's rank is "Bronze Medal".
// For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").
// Return an array answer of size n where answer[i] is the rank of the ith athlete.


struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        // Track scores
        let mut track: HashMap<i32, usize> = HashMap::new();
        for i in 0..score.len() {
            track.insert(score[i].clone(), i);
        }
        // Create return holder
        let mut ranks: Vec<String> = score.clone().into_iter()
            .map(|_| "".to_string())
            .collect();

        // Collect and Sort scores by rank
        let mut top = track.keys().collect::<Vec<&i32>>();
        top.sort_unstable_by(|a,b| b.cmp(a));

        // Iterate over scores assigning ranks
        for (rank, score) in top.iter().enumerate() {
            let player_index: usize = *track.get(&score).unwrap();
            match rank {
                0 => {
                    ranks[player_index] = "Gold Medal".to_string();
                }
                1 => {
                    ranks[player_index] = "Silver Medal".to_string();
                }
                2 => {
                    ranks[player_index] = "Bronze Medal".to_string();
                }
                _ => {
                    ranks[player_index] = (rank + 1).to_string();
                }
            }
        }
        ranks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_relative_ranks() {
        assert_eq!(Solution::find_relative_ranks(
            vec![5,4,3,2,1]), 
            vec!["Gold Medal".to_string(),"Silver Medal".to_string(),"Bronze Medal".to_string(),"4".to_string(),"5".to_string()]);
    }
    
    #[test]
    fn test_find_relative_ranks_1() {
        assert_eq!(Solution::find_relative_ranks(
            vec![10,3,8,9,4]), 
            vec!["Gold Medal".to_string(),"5".to_string(),"Bronze Medal".to_string(),"Silver Medal".to_string(),"4".to_string()]);
    }
}