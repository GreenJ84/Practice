// You are given a 0-indexed integer array batteryPercentages having length n, denoting the battery percentages of n 0-indexed devices.

// Your task is to test each device i in order from 0 to n - 1, by performing the following test operations:

// If batteryPercentages[i] is greater than 0:
// Increment the count of tested devices.
// Decrease the battery percentage of all devices with indices j in the range [i + 1, n - 1] by 1, ensuring their battery percentage never goes below 0, i.e, batteryPercentages[j] = max(0, batteryPercentages[j] - 1).
// Move to the next device.
// Otherwise, move to the next device without performing any test.
// Return an integer denoting the number of devices that will be tested after performing the test operations in order.

struct Solution;
impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        battery_percentages.iter()
          .fold(0i32, |tested, &percent| {
            if percent - tested > 0 {
              return tested + 1;
            }
            tested
          })
    }

    pub fn _count_tested_devices1(battery_percentages: Vec<i32>) -> i32 {
        let mut tested = 0;
        for idx in 0..battery_percentages.len() {
          if battery_percentages[idx] - tested > 0 {
            tested += 1;
          }
        }
        tested
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let battery_percentages = vec![1, 1, 2, 1, 3];
        let result = Solution::count_tested_devices(battery_percentages);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let battery_percentages = vec![0, 1, 2];
        let result = Solution::count_tested_devices(battery_percentages);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let battery_percentages = vec![0, 0, 0];
        let result = Solution::count_tested_devices(battery_percentages);
        assert_eq!(result, 0);
    }
}
