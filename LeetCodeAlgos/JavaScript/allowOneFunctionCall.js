// Given a function fn, return a new function that is identical to the original function except that it ensures fn is called at most once.

// The first time the returned function is called, it should return the same result as fn.
// Every subsequent time it is called, it should return undefined.

const once = function(fn) {
  let called = false;
  return function (...args) {
    if (called) return;
    called = true;
    return fn(...args);
  }
};

const fn1 = (a,b,c) => (a + b + c);
const onceFn1 = once(fn1);
const ans1 = onceFn1(1, 2, 3);
if (ans1 !== 6 || onceFn1(2, 3, 6) !== undefined) { 
  throw new Error('Test 1 failed: ' + ans1);
} else {
  console.log('Test 1 passed');
}

const fn2 = (a, b, c) => (a * b * c);
const onceFn2 = once(fn2);
const ans2 = onceFn2(5, 7, 4);
if (ans2 !== 140 || onceFn2(2, 3, 6) !== undefined || onceFn2(4, 6, 8) !== undefined) { 
  throw new Error('Test 2 failed: ' + ans2);
} else {
  console.log('Test 2 passed');
}