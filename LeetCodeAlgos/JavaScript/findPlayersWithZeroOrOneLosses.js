/** @format */

// You are given an integer array matches where matches[i] = [winneri, loseri] indicates that the player winneri defeated player loseri in a match.
// Return a list answer of size 2 where:
// answer[0] is a list of all players that have not lost any matches.
// answer[1] is a list of all players that have lost exactly one match.
// The values in the two lists should be returned in increasing order.
// Note:
// You should only consider the players that have played at least one match.
// The testcases will be generated such that no two matches will have the same outcome.

/**
 * @param {number[][]} matches
 * @return {number[][]}
 */
const findWinners = function (matches) {
  let teams = new Set();
  let lossFreq = {};
  for (let [winner, loser] of matches) {
    // Make sure winners accounted for
    teams.add(winner);
    // Make sure losers accounted for
    teams.add(loser);
    // track losses
    lossFreq[loser] = (lossFreq[loser] || 0) + 1;
  }
  let zeros = [];
  let ones = [];
  for (let team of teams) {
    if (!(team in lossFreq)) {
      zeros.push(team);
    } else if (lossFreq[team] === 1) {
      ones.push(team);
    }
  }
  return [zeros.sort((a, b) => a - b), ones.sort((a, b) => a - b)];
};

const testFindWinners = function (testNum, matches, expectation) {
  const [zeroExpect, oneExpect] = expectation;
  let [zeros, ones] = findWinners(matches);
  zeros.sort((a, b) => a - b);
  ones.sort((a, b) => a - b);

  const passed =
    zeros.toString() === zeroExpect.toString() &&
    ones.toString() === oneExpect.toString();
  console.log(`Test ${testNum} -> `, passed ? "Passed ✅" : "Failed ❌");
};

testFindWinners(
  1,
  [
    [1, 3],
    [2, 3],
    [3, 6],
    [5, 6],
    [5, 7],
    [4, 5],
    [4, 8],
    [4, 9],
    [10, 4],
    [10, 9],
  ],
  [
    [1, 2, 10],
    [4, 5, 7, 8],
  ]
);
testFindWinners(
  2,
  [
    [2, 3],
    [1, 3],
    [5, 4],
    [6, 4],
  ],
  [[1, 2, 5, 6], []]
);
