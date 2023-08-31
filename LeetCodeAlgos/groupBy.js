// Write code that enhances all arrays such that you can call the array.groupBy(fn) method on any array and it will return a grouped version of the array.

// A grouped array is an object where each key is the output of fn(arr[i]) and each value is an array containing all items in the original array with that key.

// The provided callback fn will accept an item in the array and return a string key.

// The order of each value list should be the order the items appear in the array. Any order of keys is acceptable.

// Please solve it without lodash's _.groupBy function.

Array.prototype.groupBy = function(fn) {
  const result = {};
  for (let item of this) {
    const key = fn(item);
    if (!result[key]) {
      result[key] = [item];
    } else {
      result[key].push(item);
    }
  }
  return result;
};

let test1 = [1, 2, 3, 4, 5];
test1 = test1.groupBy(item => item % 2 === 0 ? 'even' : 'odd');
console.log(test1);