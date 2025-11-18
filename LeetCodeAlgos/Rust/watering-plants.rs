// You want to water n plants in your garden with a watering can. The plants are arranged in a row and are labeled from 0 to n - 1 from left to right where the ith plant is located at x = i. There is a river at x = -1 that you can refill your watering can at.

// Each plant needs a specific amount of water. You will water the plants in the following way:
  // Water the plants in order from left to right.
  // After watering the current plant, if you do not have enough water to completely water the next plant, return to the river to fully refill the watering can.
  // You cannot refill the watering can early.

// You are initially at the river (i.e., x = -1). It takes one step to move one unit on the x-axis.

// Given a 0-indexed integer array plants of n integers, where plants[i] is the amount of water the ith plant needs, and an integer capacity representing the watering can capacity, return the number of steps needed to water all the plants.

struct Solution;
impl Solution {
  pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
    let mut steps = 0i32;
    let mut bucket = capacity;

    for x in 0..plants.len() {
      // A step for every plant we move to
      steps += 1;
      // If we can't fill plant
      // walk from previous spot (x - 1) + 1 -> x
      // walk back to curr spot (x) + 1 - previously taken step (1) -> x
      if bucket < plants[x] {
        steps += 2 * (x as usize);
        bucket = capacity;
      }
      // Water this plant
      bucket -= plants[x];
    }
    steps
  }

    pub fn watering_plants_2(plants: Vec<i32>, capacity: i32) -> i32 {
    let mut steps = 0i32;
    let mut pIdx = 0;
    let mut bucket = capacity;

    while pIdx < (plants.len() - 1) {
      // Walk to curr plant
      steps += 1;
      // Water curr plant
      bucket -= plants[pIdx];

      // Look into next plant
      pIdx += 1;
      // If can't fill, refill bucket
      if bucket < plants[pIdx] {
        steps += 2 * (pIdx as i32);
        bucket = capacity;
      }
    }
    steps + 1
  }
}