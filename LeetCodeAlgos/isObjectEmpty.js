// Given an object or an array, return if it is empty.

// An empty object contains no key-value pairs.
// An empty array contains no elements.
// You may assume the object or array is the output of JSON.parse.

const isEmpty = function(obj) {
  if (Array.isArray(obj) && obj.length === 0) {
    return true;
  }
  else if (typeof obj === 'object' && Object.keys(obj).length === 0) {
    return true;
  }
  return false;
};

obj1 = { "x": 5, "y": 42 };
isEmpty(obj1) === false ? console.log("Passed") : console.error("Failed");

obj2 = {};
isEmpty(obj2) === true ? console.log("Passed") : console.error("Failed");

obj3 = [null, false, 0];
isEmpty(obj3) === false ? console.log("Passed") : console.error("Failed");

obj4 = [];
isEmpty(obj4) === true ? console.log("Passed") : console.error("Failed");
