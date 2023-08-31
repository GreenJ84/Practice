// Given an array of asynchronous functions functions, return a new promise promise. Each function in the array accepts no arguments and returns a promise.

// promise resolves:

// When all the promises returned from functions were resolved successfully. The resolved value of promise should be an array of all the resolved values of promises in the same order as they were in the functions.
// promise rejects:

// When any of the promises returned from functions were rejected. promise should also reject with the reason of the first rejection.
// Please solve it without using the built-in Promise.all function.

const promiseAll = function (functions) {
  let results = [];
  return new Promise(async (resolve, reject) => {
    for (let i = 0; i < functions.length; i++) {
      console.log("Trying to run function " + i);
      results[i] = functions[i]();
    }

    for (let result of results) {
      try {
        console.log("Trying");
        result = await result;
      }
      catch (error) {
        console.log(error);
        reject(error);
      }
      console.log("Collected result: " + result);
    }
    resolve(results);
  })
};

// var promiseAll = async function(functions) {
//   const arr = [];
//   return new Promise(async (resolve, reject) => {
//       let arr = [], resolveCount = 0;
//       functions.map(async (fn, i) => {
//           try{
//               const res = await fn();
//               arr[i] = res;
//               resolveCount++;
//               if(functions.length === resolveCount) resolve(arr);
//           }catch(err){
//               reject(err);
//           }
//       })
//   })
// };

let x = promiseAll([asyncFunction1, asyncFunction2, asyncFunction3])
x.then(console.log)

async function asyncFunction1() {
  return new Promise(resolve => setTimeout(() => resolve("Result 1"), 1000));
}

async function asyncFunction2() {
  return new Promise((_, reject) => setTimeout(() => reject("Result 2"), 1500));
}

async function asyncFunction3() {
  return new Promise(resolve => setTimeout(() => resolve("Result 3"), 1200));
}