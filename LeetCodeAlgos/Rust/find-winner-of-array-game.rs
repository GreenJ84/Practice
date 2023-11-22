// Given an integer array arr of distinct integers and an integer k.

// A game will be played between the first two elements of the array (i.e. arr[0] and arr[1]). In each round of the game, we compare arr[0] with arr[1], the larger integer wins and remains at position 0, and the smaller integer moves to the end of the array. The game ends when an integer wins k consecutive rounds.

// Return the integer which will win the game.

// It is guaranteed that there will be a winner of the game.

struct Solution {}
impl Solution {
    pub fn get_winner(mut arr: Vec<i32>, k: i32) -> i32 {
        if k >= arr.len() as i32 {
            arr.sort_unstable();
            return *arr.last().unwrap();
        }
        let mut wins = 0;

        while wins < k {
            let mut loser = 0;
            if arr[0] > arr[1] {
                loser = arr.remove(1);
                wins += 1;
            } else {
                loser = arr.remove(0);
                wins = 1;
            }
            arr.push(loser);
        }
        arr[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_winner() {
        assert_eq!(Solution::get_winner(vec![2,1,3,5,4,6,7], 2), 5);
    }

    #[test]
    fn test_get_winner1() {
        assert_eq!(Solution::get_winner(vec![3,2,1], 10), 3);
    }

    #[test]
    fn test_get_winner2() {
        assert_eq!(Solution::get_winner(vec![1,11,22,33,44,55,66,77,88,99], 1000000000), 99);
    }
}