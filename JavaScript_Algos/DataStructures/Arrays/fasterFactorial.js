// Remember iFactorial from last chapter? Take that implementation and use a time-space tradeoff to accelerate the average running time. Recall that iFactorial(num) returns the product of positive integers from 1 to the given num. For example: ​fact(1)​ = 1, ​fact(2)​ = 2, ​fact(3) = 6. For these purposes, ​fact(0)​ = 1.

function factorial(n){
    let factorial = 1;
    for (let i in [...Array(n+1).keys()]){
        if (i != 0){
        factorial *= i;
        }
    }
    return factorial;
}

console.log(factorial(3));
console.log(factorial(4));
console.log(factorial(5));
console.log(factorial(6));