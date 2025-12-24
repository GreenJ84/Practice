// A width x height grid is on an XY-plane with the bottom-left cell at (0, 0) and the top-right cell at (width - 1, height - 1). The grid is aligned with the four cardinal directions ("North", "East", "South", and "West"). A robot is initially at cell (0, 0) facing direction "East".

// The robot can be instructed to move for a specific number of steps. For each step, it does the following.

// Attempts to move forward one cell in the direction it is facing.
// If the cell the robot is moving to is out of bounds, the robot instead turns 90 degrees counterclockwise and retries the step.
// After the robot finishes moving the number of steps required, it stops and awaits the next instruction.

// Implement the Robot class:

// Robot(int width, int height) Initializes the width x height grid with the robot at (0, 0) facing "East".
// void step(int num) Instructs the robot to move forward num steps.
// int[] getPos() Returns the current cell the robot is at, as an array of length 2, [x, y].
// String getDir() Returns the current direction of the robot, "North", "East", "South", or "West".

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::North => "North".to_string(),
            Direction::East => "East".to_string(),
            Direction::South => "South".to_string(),
            Direction::West => "West".to_string(),
        }
    }
    fn next(&mut self) {
        *self = match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        }
    }
}

struct Robot {
    width: i32,
    height: i32,
    cycle: i32,
    row: i32,
    col: i32,
    direction: Direction,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width: width - 1,
            height: height - 1,
            row: 0,
            col: 0,
            cycle: 2 * (width + height) - 4,
            direction: Direction::East,
        }
    }

    fn offset(&self) -> i32 {
        if self.row == 0 {
            self.col
        } else if self.col == self.width {
            self.width + self.row
        } else if self.row == self.height {
            self.width + self.height + (self.width - self.col)
        } else {
            self.cycle - self.row
        }
    }

    fn step(&mut self, num: i32) {
        if self.cycle == 0 {
            let turns = num.rem_euclid(4);
            for _ in 0..turns {
                self.direction.next();
            }
            return;
        }

        let new_pos = (self.offset() + num.rem_euclid(self.cycle)) % self.cycle;

        if new_pos <= self.width {
            self.row = 0;
            self.col = new_pos;
            self.direction = Direction::East;
        } else if new_pos <= self.width + self.height {
            self.row = new_pos - self.width;
            self.col = self.width;
            self.direction = Direction::North;
        } else if new_pos <= self.width + self.height + self.width {
            self.row = self.height;
            self.col = self.width - (new_pos - self.width - self.height);
            self.direction = Direction::West;
        } else {
            self.row = self.cycle - new_pos;
            self.col = 0;
            self.direction = Direction::South;
        }

        if new_pos == 0 && num > 0 {
            self.direction = Direction::South;
        }
    }

    fn step1(&mut self, num: i32) {
        let mut advance = num.rem_euclid(self.cycle);
        while advance > 0 {
            match self.direction {
                Direction::East => {
                    if self.width - self.col < advance {
                        advance -= self.width - self.col;
                        self.col = self.width;
                        self.direction.next();
                        continue;
                    }
                    self.col += advance;
                    break;
                }
                Direction::North => {
                    if self.height - self.row < advance {
                        advance -= self.height - self.row;
                        self.direction.next();
                        self.row = self.height;
                        continue;
                    }
                    self.row += advance;
                    break;
                }
                Direction::West => {
                    if self.col < advance {
                        advance -= self.col;
                        self.col = 0;
                        self.direction.next();
                        continue;
                    }
                    self.col -= advance;
                    break;
                }
                Direction::South => {
                    if self.row < advance {
                        advance -= self.row;
                        self.row = 0;
                        self.direction.next();
                        continue;
                    }
                    self.row -= advance;
                    break;
                }
            }
        }
        if num > 0 && advance == 0 && self.col == 0 && self.direction == Direction::East {
            self.direction = Direction::South;
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        vec![self.col, self.row]
    }

    fn get_dir(&self) -> String {
        self.direction.to_string()
    }
}

/**
 * The Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut robot = Robot::new(6, 3);
        robot.step(2);
        robot.step(2);
        assert_eq!(robot.get_pos(), vec![4, 0]);
        assert_eq!(robot.get_dir(), "East");
        robot.step(2);
        robot.step(1);
        robot.step(4);
        assert_eq!(robot.get_pos(), vec![1, 2]);
        assert_eq!(robot.get_dir(), "West");
    }

    #[test]
    fn test_initial_position() {
        let robot = Robot::new(10, 10);
        assert_eq!(robot.get_pos(), vec![0, 0]);
        assert_eq!(robot.get_dir(), "East");
    }

    #[test]
    fn test_move_to_boundary() {
        let mut robot = Robot::new(3, 3);
        robot.step(2);
        assert_eq!(robot.get_pos(), vec![2, 0]);
        assert_eq!(robot.get_dir(), "East");
    }

    #[test]
    fn test_full_rotation() {
        let mut robot = Robot::new(5, 5);
        robot.step(4);
        robot.step(4);
        robot.step(4);
        robot.step(4);
        assert_eq!(robot.get_pos(), vec![0, 0]);
        assert_eq!(robot.get_dir(), "South");
        robot.step(1);
        assert_eq!(robot.get_pos(), vec![1, 0]);
        assert_eq!(robot.get_dir(), "East");
    }

    #[test]
    fn test_spiral_movement() {
        let mut robot = Robot::new(10, 10);
        robot.step(3);
        robot.step(3);
        robot.step(3);
        robot.step(3);
        assert_eq!(robot.get_pos(), vec![9, 3]);
    }

    #[test]
    fn test_min_grid() {
        let mut robot = Robot::new(2, 2);
        robot.step(1);
        robot.step(1);
        assert_eq!(robot.get_pos(), vec![1, 1]);
    }
}
