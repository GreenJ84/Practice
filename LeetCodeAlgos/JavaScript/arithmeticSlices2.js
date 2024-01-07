// Given an integer array nums, return the number of all the arithmetic subsequences of nums.
// A sequence of numbers is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
  //# For example, [1, 3, 5, 7, 9], [7, 7, 7, 7], and [3, -1, -5, -9] are arithmetic sequences.
  //# For example, [1, 1, 2, 5, 7] is not an arithmetic sequence.
// A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
  //# For example, [2,5,10] is a subsequence of [1,2,1,2,4,1,5,10].
// The test cases are generated so that the answer fits in 32-bit integer.


/**
 * @param {number[]} nums
 * @return {number}
 */
const numberOfArithmeticSlices = function (nums) {
  const n = nums.length; // Number of elements in the array
  const dp = new Array(n).fill().map(() => new Map()); // 2-D frequence table
  let slices = 0; // Arithmetic subsequences

  // Look at all elements in the array
  for (let currEl = 1; currEl < n; currEl++) {
    // Look at all elements previous to the current Element
    for (let prevEl = 0; prevEl < currEl; prevEl++) {
      // Get difference between current and previous
      const diff = nums[currEl] - nums[prevEl];
      // Get number of times previous Element has seen the difference
      let prevFrequency = dp[prevEl].get(diff) || 0;
      // Get number of times current Element has seen the difference
      let currFrequency = dp[currEl].get(diff) || 0;
      // Update the frequence table with the prevent frequency
      slices += prevFrequency
      // Update the current frequency with what has been seen, currently seen, and the current pair
      dp[currEl].set(diff, prevFrequency + currFrequency + 1);
    }
  }
  // Return the number of arithmetic subsequences
  return slices;
};

console.log(
  "Test 1: ",
  numberOfArithmeticSubsequences([2,4,6,8,10]) === 7 ? "Passed!" : "Failed!"
);

console.log(
  "Test 2: ",
  numberOfArithmeticSubsequences([7, 7, 7, 7, 7]) === 16 ? "Passed!" : "Failed!"
);
