// There is a task management system that allows users to manage their tasks, each associated with a priority. The system should efficiently handle adding, modifying, executing, and removing tasks.

// Implement the TaskManager class:

// TaskManager(vector<vector<int>>& tasks) initializes the task manager with a list of user-task-priority triples. Each element in the input list is of the form [userId, taskId, priority], which adds a task to the specified user with the given priority.

// void add(int userId, int taskId, int priority) adds a task with the specified taskId and priority to the user with userId. It is guaranteed that taskId does not exist in the system.

// void edit(int taskId, int newPriority) updates the priority of the existing taskId to newPriority. It is guaranteed that taskId exists in the system.

// void rmv(int taskId) removes the task identified by taskId from the system. It is guaranteed that taskId exists in the system.

// int execTop() executes the task with the highest priority across all users. If there are multiple tasks with the same highest priority, execute the one with the highest taskId. After executing, the taskId is removed from the system. Return the userId associated with the executed task. If no tasks are available, return -1.

// Note that a user may be assigned multiple tasks.

// Constraints:
// 1 <= tasks.length <= 10^5
// 0 <= userId <= 10^5
// 0 <= taskId <= 10^5
// 0 <= priority <= 10^9
// 0 <= newPriority <= 10^9
// At most 2 * 10^5 calls will be made in total to add, edit, rmv, and execTop methods.
// The input is generated such that taskId will be valid.
use std::collections::{BinaryHeap, HashMap};
struct TaskManager {
    tasks: HashMap<i32, (i32, i32)>, // taskId -> (userId, priority)
    priority_queue: BinaryHeap<(i32, i32, i32)>, // (priority, taskId, userId)
}
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_manager = Self {
            tasks: HashMap::new(),
            priority_queue: BinaryHeap::new(),
        };
        for task in tasks {
            let (user_id, task_id, priority) = (task[0], task[1], task[2]);
            task_manager.add(user_id, task_id, priority);
        }
        task_manager
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.insert(task_id, (user_id, priority));
        self.priority_queue.push((priority, task_id, user_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let &(user_id, _) = self.tasks.get(&task_id).unwrap();
        self.tasks.insert(task_id, (user_id, new_priority));
        self.priority_queue.push((new_priority, task_id, user_id));
    }

    fn rmv(&mut self, task_id: i32) {
        self.tasks.remove(&task_id);
    }

    fn exec_top(&mut self) -> i32 {
        while let Some((priority, task_id, user_id)) = self.priority_queue.pop() {
            if self.tasks.get(&task_id) == Some(&(user_id, priority)) {
                self.tasks.remove(&task_id);
                return user_id;
            }
        }
        -1
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut task_manager =
            TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
        task_manager.add(4, 104, 5);
        task_manager.edit(102, 8);
        assert_eq!(task_manager.exec_top(), 3);
        task_manager.rmv(101);
        task_manager.add(5, 105, 15);
        assert_eq!(task_manager.exec_top(), 5);
    }

    #[test]
    fn test2() {
        let mut task_manager = TaskManager::new(vec![vec![1, 201, 30], vec![2, 202, 25]]);
        task_manager.add(3, 203, 35);
        task_manager.edit(202, 40);
        assert_eq!(task_manager.exec_top(), 2);
        task_manager.rmv(201);
        assert_eq!(task_manager.exec_top(), 3);
    }
}
