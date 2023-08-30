// Write code that enhances all arrays such that you can call the array.last() method on any array and it will return the last element. If there are no elements in the array, it should return -1.

// You may assume the array is the output of JSON.parse.


Array.prototype.last = function () {
  if (this.length === 0) return -1;
  return this[this.length - 1];
};

[1, 2, 3, 4, 5].last() === 5 ?
  console.log('Passed Test1') :
  console.log('Failed Test1');

['1', {2: 3}, 4, true].last() === true ?
  console.log('Passed Test2') :
    console.log('Failed Test2');
  
[].last() === -1 ?
  console.log('Passed Test3') :
  console.log('Failed Test3');
  
[4].last() === 4 ?
  console.log('Passed Test4') :
  console.log('Failed Test4');