// Write a function argumentsLength that returns the count of arguments passed to it.

const argumentsLength = function(...args) {
  return args.length;
};

const test1 = argumentsLength(5);
if (test1 !== 1) {
  throw new Error(`FALSE: expected 3, got ${test1}`);
} else {
  console.log("Passed Test 1");
}

const test2 = argumentsLength({}, null, "3");
if (test2 !== 3) { 
  throw new Error(`FALSE: expected 3, got ${test2}`);
} else {
  console.log("Passed Test 2");
}

const test3 = argumentsLength(1, 2, 3, 4);
if (test3 !== 4) { 
  throw new Error(`FALSE: expected 3, got ${test3}`);
} else {
  console.log("Passed Test 3");
}