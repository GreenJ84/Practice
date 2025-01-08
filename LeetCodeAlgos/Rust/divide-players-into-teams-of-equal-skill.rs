// You are given a positive integer array skill of even length n where skill[i] denotes the skill of the ith player. Divide the players into n / 2 teams of size 2 such that the total skill of each team is equal.
//
// The chemistry of a team is equal to the product of the skills of the players on that team.
//
// Return the sum of the chemistry of all the teams, or return -1 if there is no way to divide the players into teams such that the total skill of each team is equal.


struct Solution {}
impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let n = skill.len();
        let teams = n / 2;

        let mut target: i64 = skill.iter().map(|&x| x as i64).sum();
        if target % teams as i64 != 0 {
            return -1;
        }
        target /= teams as i64;

        skill.sort();
        let mut chemistry: i64 = 0;
        for idx in 0..n/2 {
            let first = skill[idx];
            let last = skill[n - 1 - idx];
            if i64::from(first + last) != target {
                return -1;
            }
            chemistry += (first * last) as i64;
        }
        chemistry
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            Solution::divide_players(Vec::from([3,2,5,1,3,4])),
            22
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            Solution::divide_players(Vec::from([3,4])),
            12
        );
    }

    #[test]
    fn test3(){
        assert_eq!(
            Solution::divide_players(Vec::from([1,1,2,3])),
            -1
        );
    }
}