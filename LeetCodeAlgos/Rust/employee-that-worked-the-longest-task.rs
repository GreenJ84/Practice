// There are n employees, each with a unique id from 0 to n - 1.
// You are given a 2D integer array logs where logs[i] = [idi, leaveTimei] where:
// id[i] is the id of the employee that worked on the ith task,
// leaveTime[i] is the time at which the employee finished the ith task. All the values leaveTime[i] are unique.
// Note that the ith task starts the moment right after the (i - 1)th task ends, and the 0th task starts at time 0.
// Return the id of the employee that worked the task with the longest time. If there is a tie between two or more employees, return the smallest id among them.

struct Solution {}
use std::cmp::min;
impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut ans: Vec<i32> = logs[0].to_owned();
        for idx in 1..logs.len() {
            let curr_task = vec![
                logs[idx][0],                    // id
                logs[idx][1] - logs[idx - 1][1], // Task time
            ];
            if curr_task[1] > ans[1] {
                ans = curr_task;
            } else if curr_task[1] == ans[1] {
                ans[0] = min(ans[0], curr_task[0]);
            }
        }
        ans[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardest_worker1() {
        assert_eq!(
            Solution::hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]),
            1
        )
    }

    #[test]
    fn hardest_worker2() {
        assert_eq!(
            Solution::hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]),
            3
        )
    }

    #[test]
    fn hardest_worker3() {
        assert_eq!(
            Solution::hardest_worker(2, vec![vec![0, 10], vec![1, 20]]),
            0
        )
    }
}
