// In a town, there are n people labeled from 1 to n. There is a rumor that one of these people is secretly the town judge.
// If the town judge exists, then:
// The town judge trusts nobody.
// Everybody (except for the town judge) trusts the town judge.
// There is exactly one person that satisfies properties 1 and 2.
// You are given an array trust where trust[i] = [ai, bi] representing that the person labeled ai trusts the person labeled bi. If a trust relationship does not exist in trust array, then such a trust relationship does not exist.
// Return the label of the town judge if the town judge exists and can be identified, or return -1 otherwise.

//! Constraints
// 1 <= n <= 1000
// 0 <= trust.length <= 104
// trust[i].length == 2
// All the pairs of trust are unique.
// ai != bi
// 1 <= ai, bi <= n

struct Solution {}
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trusts_somebody = vec![false; n as usize];
        let mut trust_count = vec![0; n as usize];
        for trust_bond in trust {
            trusts_somebody[trust_bond[0] as usize - 1] = true;
            trust_count[trust_bond[1] as usize - 1] += 1;
        }
        let mut ans = -1;
        for i in 0..trust_count.len() {
            if !trusts_somebody[i] && trust_count[i] == n as usize - 1 {
                if ans != -1 {
                    return -1;
                }
                ans = i as i32 + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_judge1() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn test_find_judge2() {
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }

    #[test]
    fn test_find_judge3() {
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
    }

    #[test]
    fn test_find_judge4() {
        assert_eq!(Solution::find_judge(1, vec![]), 1);
    }
}
