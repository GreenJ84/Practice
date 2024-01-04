// You are given a 0-indexed array nums consisting of positive integers.
// There are two types of operations that you can apply on the array any number of times:
  // Choose two elements with equal values and delete them from the array.
  // Choose three elements with equal values and delete them from the array.
// Return the minimum number of operations required to make the array empty, or -1 if it is not possible.

/**
 * @param {number[]} nums
 * @return {number}
 */
var minOperations = function (nums) {
  // Track the number of times each number appears
  const tracker = {};
  // Iterate through the array
  for (let num of nums) {
    // If the number is not in the tracker, add it
    if (!(num in tracker)) tracker[num] = 1;
    // If the number is in the tracker, increment its count
    else tracker[num]++;
  }
  let ops = 0;
  // Iterate through the tracker
  for (let count of Object.values(tracker)) {
    // If the count is 1, empty is not possible
    if (count === 1) {
        ops = -1;
        break;
    }
    // Get minimum number of 3 sets possible
    let numOps = Math.floor(count / 3);
    // Add an extra operation if 3 doesnt divide evenly
    if (count % 3 != 0) numOps++;
    // Add the number of operations for this num
    ops += numOps
  }
  return ops;
};

console.log("Test Case 1: ", minOperations([2, 3, 3, 2, 2, 4, 2, 3, 4]) === 4 ? "Pass" : "Fail");
console.log("Test Case 1: ", minOperations([2,1,2,2,3,3]) === -1 ? "Pass" : "Fail");