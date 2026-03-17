// You are given a 2D integer array logs where each logs[i] = [birthi, deathi] indicates the birth and death years of the ith person.

// The population of some year x is the number of people alive during that year. The ith person is counted in year x's population if x is in the inclusive range [birthi, deathi - 1]. Note that the person is not counted in the year that they die.

// Return the earliest year with the maximum population.

struct Solution;
impl Solution {
    pub fn maximum_population(mut logs: Vec<Vec<i32>>) -> i32 {
        logs.sort_unstable_by_key(|log| log[0]);

        let (mut min_year, mut max_pop, mut cur_pop) = (2051, 0, 0);
        let mut heap = std::collections::BinaryHeap::<std::cmp::Reverse<i32>>::new();

        for log in logs {
          while !heap.is_empty() && heap.peek().unwrap().0 <= log[0] {
            cur_pop -= 1;
            heap.pop();
          }
          cur_pop += 1;
          if cur_pop > max_pop{
            max_pop = cur_pop;
            min_year = log[0];
          }
          heap.push(std::cmp::Reverse(log[1]));
        }
        min_year
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::maximum_population(vec![vec![1993,1999],vec![2000,2010]]), 1993);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::maximum_population(vec![vec![1950,1961],vec![1960,1971],vec![1970,1981]]), 1960);
    }
}