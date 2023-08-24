// Given an array of functions [f1, f2, f3, ..., fn], return a new function fn that is the function composition of the array of functions.

// The function composition of [f(x), g(x), h(x)] is fn(x) = f(g(h(x))).

// The function composition of an empty list of functions is the identity function f(x) = x.

// You may assume each function in the array accepts one integer as input and returns one integer as output.

var compose = function(functions) {
  return function (x) {
    let val = x;
    for (let i = functions.length - 1; i >= 0; i--) { 
      val = functions[i](val);
    }
    return val;
  }
};

function add2(x) {
  return x + 2;
}

function double(x) {
  return x * 2;
}

const transformation = compose([add2, double]);
console.log(transformation(3));