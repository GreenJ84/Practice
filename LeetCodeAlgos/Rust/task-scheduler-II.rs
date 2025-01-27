// You are given a 0-indexed array of positive integers tasks, representing tasks that need to be completed in order, where tasks[i] represents the type of the ith task.
//
// You are also given a positive integer space, which represents the minimum number of days that must pass after the completion of a task before another task of the same type can be performed.
//
// Each day, until all tasks have been completed, you must either:
//
// Complete the next task from tasks, or
// Take a break.
// Return the minimum number of days needed to complete all tasks.


struct Solution{}

use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let curr_day = 1;
        let mgr: HashMap<i32, i32> = HashMap::new();
        for task in &tasks {
            if let Some(last_day) = mgr.get_mut(task) {

            } else {
                mgr.insert(task, curr_day);
            }
        }

    }
}