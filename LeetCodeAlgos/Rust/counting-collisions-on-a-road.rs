// There are n cars on an infinitely long road. The cars are numbered from 0 to n - 1 from left to right and each car is present at a unique point.

// You are given a 0-indexed string directions of length n. directions[i] can be either 'L', 'R', or 'S' denoting whether the ith car is moving towards the left, towards the right, or staying at its current point respectively. Each moving car has the same speed.

// The number of collisions can be calculated as follows:
// - When two cars moving in opposite directions collide with each other, the number of collisions increases by 2.
// - When a moving car collides with a stationary car, the number of collisions increases by 1.

// After a collision, the cars involved can no longer move and will stay at the point where they collided. Other than that, cars cannot change their state or direction of motion.

// Return the total number of collisions that will happen on the road.

struct Solution;
impl Solution {
  pub fn count_collisions(directions: String) -> i32 {
    let mut result = 0i32;

    let mut moving_right = 0i32;
    let mut road_blocked = false;

    for ch in directions.chars() {
      match ch {
        'R' => { moving_right += 1; },
        'L' => {
          if moving_right > 0 {
            if !road_blocked { road_blocked = true; }
            result += moving_right + 1;
            moving_right = 0;
          } else if road_blocked {
            result += 1;
          }
        },
        'S' => {
          if !road_blocked { road_blocked = true; }
          if moving_right > 0 {
            result += moving_right;
            moving_right = 0;
          }
        },
        _ => panic!("Invalid algorithm input; does not comply with constraints.")
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn example_1() {
    assert_eq!(Solution::count_collisions("RLRSLL".to_string()), 5);
  }

  #[test]
  fn example_2() {
    assert_eq!(Solution::count_collisions("LLRR".to_string()), 0);
  }

  #[test]
  fn single_values() {
    assert_eq!(Solution::count_collisions("L".to_string()), 0);
    assert_eq!(Solution::count_collisions("S".to_string()), 0);
    assert_eq!(Solution::count_collisions("R".to_string()), 0);
  }

  #[test]
  fn opposite_pair_rl() {
    assert_eq!(Solution::count_collisions("RL".to_string()), 2);
  }

  #[test]
  fn group_collision_rrll() {
    // R R L L -> middle R and L collide (2), then outer R and L hit the stopped cars (+1 each) = 4
    assert_eq!(Solution::count_collisions("RRLL".to_string()), 4);
  }

  #[test]
  fn right_into_stationary_rrs() {
    // R R S -> second R hits S (+1), first R then hits stopped car (+1) = 2
    assert_eq!(Solution::count_collisions("RRS".to_string()), 2);
  }

  #[test]
  fn all_rights_no_collision() {
    assert_eq!(Solution::count_collisions("RRR".to_string()), 0);
  }

  #[test]
  fn all_lefts_no_collision() {
    assert_eq!(Solution::count_collisions("LLL".to_string()), 0);
  }

  #[test]
  fn stationary_between_opposites_srl() {
    // S R L -> R and L collide (+2), S unaffected
    assert_eq!(Solution::count_collisions("SRL".to_string()), 2);
  }
}
