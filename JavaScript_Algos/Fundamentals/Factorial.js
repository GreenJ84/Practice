// Write a function ​factorial(num)​ that, given a number, returns the product (multiplication) of all positive integers from 1 up to number (inclusive). For example, ​factorial(3)​ = 6 (or 1 * 2 * 3); factorial(5)​ = 120 (or 1 * 2 * 3 * 4 * 5).

function factorial(num){
    let product = 1;
    while (num > 0){
        product *= num
        num--
    }
    return product;
}

console.log(factorial(3));
console.log(factorial(5));