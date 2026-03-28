// Given a list of dominoes, dominoes[i] = [a, b] is equivalent to dominoes[j] = [c, d] if and only if either (a == c and b == d), or (a == d and b == c) - that is, one domino can be rotated to be equal to another domino.

// Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length, and dominoes[i] is equivalent to dominoes[j].

struct Solution;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::<Vec<i32>, i32>::new();
        let mut ans = 0;
        for d in dominoes {
            let mut d = d;
            d.sort();
            let en = map.entry(d).or_insert(0);
            if en > &mut 0 {
                ans += *en;
            }
            *en += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
            1
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2]
            ]),
            3
        );
    }
}
