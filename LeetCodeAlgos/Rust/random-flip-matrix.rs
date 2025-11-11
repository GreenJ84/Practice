// cargo-deps: rand = "0.9.0-alpha.0"

// There is an m x n binary grid matrix with all the values set 0 initially. Design an algorithm to randomly pick an index (i, j) where matrix[i][j] == 0 and flips it to 1. All the indices (i, j) where matrix[i][j] == 0 should be equally likely to be returned.

// Optimize your algorithm to minimize the number of calls made to the built-in random function of your language and optimize the time and space complexity.

// Implement the Solution class:

// Solution(int m, int n) Initializes the object with the size of the binary matrix m and n.
// int[] flip() Returns a random index [i, j] of the matrix where matrix[i][j] == 0 and flips it to 1.
// void reset() Resets all the values of the matrix to be 0.

use rand::prelude::*;
use std::collections::HashSet;

struct Solution {
  m: u32,
  n: u32,
  flipped: HashSet<(i32, i32)>,
  rand: ThreadRng
}

impl Solution {
  fn new(m: i32, n: i32) -> Self {
    Self {
      m: m as u32,
      n: n as u32,
      flipped: HashSet::new(),
      rand: rand::thread_rng()
    }
  }

  fn flip(&mut self) -> Vec<i32> {
    let row = self.rand.gen_range(0..self.m) as i32;
    let col = self.rand.gen_range(0..self.n) as i32;
    if self.flipped.contains(&(row, col)) {
      return self.flip();
    }
    self.flipped.insert((row, col));
    vec![row, col]
  }

  fn reset(&mut self) {
    self.flipped.clear();
  }
}