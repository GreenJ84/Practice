// You are given an integer array nums. You need to create a 2D array from nums satisfying the following conditions:
  // The 2D array should contain only the elements of the array nums.
  // Each row in the 2D array contains distinct integers.
  // The number of rows in the 2D array should be minimal.
  // Return the resulting array. If there are multiple answers, return any of them.
// Note that the 2D array can have a different number of elements on each row.

/**
 * @param {number[]} nums
 * @return {number[][]}
 */
const findMatrix = function(nums) {
  let res = [];
  while (nums.length > 0){
      let lvl = [];
      for (let idx = 0; idx < nums.length; idx++) {
          if (!lvl.includes(nums[idx])){
              lvl.push(nums[idx]);
              nums.splice(idx, 1);
              idx--;
          }
      }
      res.push(lvl);
  }
  return res;
};

function testFindMatrix(nums, expected) { 
  return JSON.stringify(findMatrix(nums)) === JSON.stringify(expected)
}

console.log(
  `Test 1 ${testFindMatrix([1,3,4,1,2,3,1], [[1,3,4,2],[1,3],[1]]) ? "passed" : "failed"}`
);
console.log(
  `Test 2 ${testFindMatrix([1,2,3,4], [[1,2,3,4]])? "passed" : "failed"}`
);