// You are given an integer array ranks and a character array suits. You have 5 cards where the ith card has a rank of ranks[i] and a suit of suits[i].

// The following are the types of poker hands you can make from best to worst:

// "Flush": Five cards of the same suit.
// "Three of a Kind": Three cards of the same rank.
// "Pair": Two cards of the same rank.
// "High Card": Any single card.
// Return a string representing the best type of poker hand you can make with the given cards.

// Note that the return values are case-sensitive.

struct Solution;
impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let (mut f_pos, f) = (true, suits[0]);
        let mut r = [0; 13];
        for i in 0..5{
          if suits[i] != f { f_pos = false; }
          r[(ranks[i] - 1) as usize] += 1;
        }

        if f_pos { return "Flush".into(); }
        return match r.iter().max().unwrap() {
          x if x > &2 => "Three of a Kind".into(),
          2 => "Pair".into(),
          _ => "High Card".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ranks = vec![13,2,3,1,9];
        let suits = vec!['a','a','a','a','a'];
        assert_eq!(Solution::best_hand(ranks, suits), "Flush");
    }

    #[test]
    fn test2() {
        let ranks = vec![4,4,2,4,4];
        let suits = vec!['d','a','a','b','c'];
        assert_eq!(Solution::best_hand(ranks, suits), "Three of a Kind");
    }

    #[test]
    fn test3() {
        let ranks = vec![10,10,2,12,9];
        let suits = vec!['a','b','c','a','d'];
        assert_eq!(Solution::best_hand(ranks, suits), "Pair");
    }

    #[test]
    fn test4() {
        let ranks = vec![1,2,3,4,5];
        let suits = vec!['a','b','c','d','e'];
        assert_eq!(Solution::best_hand(ranks, suits), "High Card");
    }
}