// Fibonacci numbers have been studied for years and appear often in nature. Write a function that will return an array of Fibonacci numbers up to a given length n. 
//Fibonacci numbers are calculated by adding the last two values in the sequence together. So if the 4th value is 2 and the 5th value is 3 then the next value in the sequence is 5.

function fibonacciArray(n) {
    var fibArr = []
    n >= 2 ? fibArr = [0, 1] : n = 1 ? fibArr = [0] : '';
    let x = 2
    while(x < n){
        fibArr[x] = fibArr[x-1] + fibArr[x-2];
        x++;
    }
    return fibArr;
}

var result = fibonacciArray(10);
console.log(result); // we expect back [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]