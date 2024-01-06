// Given an integer array nums, return the length of the longest strictly increasing subsequence

/**
 * @param {number[]} nums
 * @return {number}
 */
var lengthOfLIS = function(nums) {
  let n = nums.length;
  let dp = new Array(n).fill(1);

  for (let i = 0; i < n; i++){
      for (let j = 0; j < i; j++){
          if (nums[i]>nums[j]){
              dp[i]= Math.max(dp[i],1+dp[j])
          }
      }
  }
  return Math.max(...dp);
};

console.log(
  "Test 1: ",
  lengthOfLIS([10,9,2,5,3,7,101,18]) === 4 ? "Passed!" : "Failed!"
);
console.log(
  "Test 2: ",
  lengthOfLIS([0,1,0,3,2,3]) === 4 ? "Passed!" : "Failed!"
);
console.log(
  "Test 3: ",
  lengthOfLIS([7,7,7,7,7,7,7]) === 1 ? "Passed!" : "Failed!"
);