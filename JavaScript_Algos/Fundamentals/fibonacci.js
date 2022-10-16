// Implement the Fibonacci function, a famous mathematical equation that generates a numerical sequence such that each number is the sum of the previous two. The Fibonacci numbers at index 0 and 1, coincidentally, have values of 0 and 1. Your function should accept an argument of which Fibonacci number.
// Examples: ​fibonacci(2)​ = 1, ​fibonacci(3)​ = 2, ​fibonacci(4)​ = 3, ​fibonacci(5)​ = 5, etc.

function fibbonacci(num){
    let sum = 0;
    if (num < 2) { return sum }
    else if (num == 2){ return 1 }
    else{
    let i = 2;
    let one = 1;
    let two = 0;
    let temp = 0;
    while (i <= num){
        sum = one + two;
        console.log(one+'-one + '+two+'-two = '+sum+'-sum');
        temp = one;
        one = sum;
        two = temp;
        i++;
    }}
    return sum;
}

console.log(fibbonacci(2));
console.log(fibbonacci(3));
console.log(fibbonacci(4));
console.log(fibbonacci(5));
console.log(fibbonacci(10));