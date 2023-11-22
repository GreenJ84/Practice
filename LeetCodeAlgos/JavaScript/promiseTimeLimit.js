// Given an asynchronous function fn and a time t in milliseconds, return a new time limited version of the input function. fn takes arguments provided to the time limited function.

// The time limited function should follow these rules:

// If the fn completes within the time limit of t milliseconds, the time limited function should resolve with the result.
// If the execution of the fn exceeds the time limit, the time limited function should reject with the string "Time Limit Exceeded".

const timeLimit = function(fn, t) {
	return async function(...args) {
        return new Promise(async (resolve, reject) => {
            setTimeout(() => reject("Time Limit Exceeded"), t);
            try {
                resolve(await fn(...args));
            }
            catch(err) {
                reject(err)
            }
        })
    }
};