// Given an array arr and a function fn, return a sorted array sortedArr. You can assume fn only returns numbers and those numbers determine the sort order of sortedArr. sortedArray must be sorted in ascending order by fn output.

// You may assume that fn will never duplicate numbers for a given array.

const sortBy = function(arr, fn) {
  return arr.sort((a, b) => fn(a) - fn(b));
};

// const sortBy = function(arr, fn) {
//   return partition(arr, fn);
// };

// function partition(arr, fn) { 
//   if (arr.length <= 1) {
//     return arr;
//   }
//   let mid = Math.floor(arr.length / 2);
//   let left = arr.slice(0, mid);
//   let right = arr.slice(mid, arr.length);

//   left = partition(left);
//   right = partition(right);

//   return merge(left, right, fn);
// }

// function merge(arr1, arr2, fn) {
//   let result = [];
//   let leftIdx = 0;
//   let rightIdx = 0;

//   while (leftIdx < arr1.length && rightIdx < arr2.length) {
//     if ( fn(arr1[leftIdx]) < fn(arr2[rightIdx]) ) { 
//       result.push(arr1[leftIdx]);
//       leftIdx++;
//     } else {
//       result.push(arr2[rightIdx]);
//       rightIdx++;
//     }
//   }
//   return result.concat(arr1.slice(leftIdx), arr2.slice(rightIdx));
// }