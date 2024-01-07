// We have n jobs, where every job is scheduled to be done from startTime[i] to endTime[i], obtaining a profit of profit[i].
// You're given the startTime, endTime and profit arrays, return the maximum profit you can take such that there are no two jobs in the subset with overlapping time range.
// If you choose a job that ends at time X you will be able to start another job that starts at time X.

/**
 * @param {number[]} startTime
 * @param {number[]} endTime
 * @param {number[]} profit
 * @return {number}
 */
const jobScheduling = function(startTime, endTime, profit) {
  let n = profit.length;
  let jobs = new Array(n)
    .fill(0)
    .map((_, idx) => { return [startTime[idx], endTime[idx], profit[idx]] })
    .sort((a, b) => a[1] - b[1]);
  let dp = new Array(n + 1).fill(0);

  for (let idx = 0; idx < n; idx++){
    const [start, end, profit] = jobs[idx];
    
    const nonConflict = scheduleCheck(jobs, idx, start);
    dp[idx + 1] = Math.max(dp[idx], dp[nonConflict] + profit);
  }
  console.log(dp);
  return dp[n];
};

const scheduleCheck = (jobs, endIdx, targetStart) => { 
  let low = 0;
  let high = endIdx;
  while (low < high) { 
    const mid = Math.floor((low + high) / 2);
    const start = jobs[mid][1];
    if (start <= targetStart) {
      low = mid + 1;
    } else {
      high = mid;
    }
  }
  return low;
}

console.log(
  "Test 1: ",
  jobScheduling([1,2,3,3], [3,4,5,6], [50,10,40,70]) === 120 ? "Passed!" : "Failed!"
);
console.log(
  "Test 2: ",
  jobScheduling([1,2,3,4,6], [3,5,10,6,9], [20,20,100,70,60]) === 150 ? "Passed!" : "Failed!"
);
console.log(
  "Test 3: ",
  jobScheduling([1,1,1], [2,3,4], [5,6,4]) === 6 ? "Passed!" : "Failed!"
);