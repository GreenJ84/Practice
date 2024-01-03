// Anti-theft security devices are activated inside a bank. You are given a 0-indexed binary string array bank representing the floor plan of the bank, which is an m x n 2D matrix. bank[i] represents the ith row, consisting of '0's and '1's. '0' means the cell is empty, while'1' means the cell has a security device.
// There is one laser beam between any two security devices if both conditions are met:
  // The two devices are located on two different rows: r1 and r2, where r1 < r2.
  // For each row i where r1 < i < r2, there are no security devices in the ith row.
  // Laser beams are independent, i.e., one beam does not interfere nor join with another.
// Return the total number of laser beams in the bank.

/**
 * @param {string[]} bank
 * @return {number}
 */
const numberOfBeams = function(bank) {
  let res = 0;
  let firstHold = 0, secondHold = 0;
  for (let row of bank){
      // Collect # of devices per row
      for (let cell of row){
          if(cell === '1') secondHold++;
      }
      // Have to have a row with devices before adding
      if (firstHold > 0) {
          res += firstHold * secondHold;
      }
      // Must have a row with devices before swapping
      if (secondHold > 0){
          [firstHold, secondHold] = [secondHold, 0];
      }
  }
  return res;
};

console.log(
  `Test 1 ${numberOfBeams(["011001","000000","010100","001000"]) === 8 ? "passed" : "failed"}`
);
console.log(
  `Test 2 ${numberOfBeams(["000","111","000"]) === 0 ? "passed" : "failed"}`
);