// Given two promises promise1 and promise2, return a new promise. promise1 and promise2 will both resolve with a number. The returned promise should resolve with the sum of the two numbers.

async function addTwoPromises(promise1: Promise<number>, promise2: Promise<number>): Promise<number> {
  return Promise.all([promise1, promise2]).then(([res1, res2]) => { 
    return Promise.resolve(res1 + res2);
  })
};

/**
 * addTwoPromises(Promise.resolve(2), Promise.resolve(2))
 *   .then(console.log); // 4
 */