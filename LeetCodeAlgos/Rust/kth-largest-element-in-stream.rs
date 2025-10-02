// You are part of a university admissions office and need to keep track of the kth highest test score from applicants in real-time. This helps to determine cut-off marks for interviews and admissions dynamically as new applicants submit their scores.

// You are tasked to implement a class which, for a given integer k, maintains a stream of test scores and continuously returns the kth highest test score after a new score has been submitted. More specifically, we are looking for the kth highest score in the sorted list of all scores.

use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct KthLargest {
  k: i32,
  heap: BinaryHeap<Reverse<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
      let mut new = Self {
        k,
        heap: BinaryHeap::<Reverse<i32>>::with_capacity(k as usize)
      };
      for num in nums {
        new.add(num);
      }
      new
    }

    fn add(&mut self, val: i32) -> i32 {
      match self.heap.len() == self.heap.capacity() {
      true => {
        if &Reverse(val) > self.heap.peek().unwrap() {
          self.heap.pop();
          self.heap.push(Reverse(val));
        }
      }
      false => { self.heap.push(Reverse(val)); }
    }
    self.heap.peek().unwrap().0
  }
}