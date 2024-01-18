// You are climbing a staircase. It takes n steps to reach the top.

// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

/**
 * @param {number} n
 * @return {number}
 */
const climbStairs = function (n) {
  if (n == 0) return 0;
  let firstStep = 1;
  if(n == 1) return firstStep;
  let secondStep = 2;
  if (n == 2) return secondStep;
  for (let i = 3; i <= n; i++) {
    [firstStep, secondStep] = [secondStep, firstStep + secondStep];
  }
  return secondStep;
};

const testClimbStairs = function (testNum, staircase, expected) { 
  const result = climbStairs(staircase);
  console.log(`Test #${testNum}: ${result} -> `, result === expected ? 'Passed ✅' : 'Failed ❌');
}

testClimbStairs(1, 2, 2);
testClimbStairs(2, 3, 3);