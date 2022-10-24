// Use a time-space tradeoff to accelerate the average running time of an iFibonacci(num) function that returns the ‘num’th number in the Fibonacci sequence. Recall: fib(0) = 0, fib(1) = 1, fib(2) = 1, fib(3) = 2.

function fabFibonacci(n){
        let first, second;
    
        if (n < 0){
            return null;
        }
        else if (n === 0){
            return 0;
        } 
        else if (n == 1 ){
            return 1
        }
        else{
            let i = 2
            first = 0;
            second = 1;
            while (i <= n){
                let temp = first + second;
                first = second;
                second = temp;
                i++;
            }
            return second;
        }
    }

console.log(fabFibonacci(2));
console.log(fabFibonacci(3));
console.log(fabFibonacci(4));
console.log(fabFibonacci(5));
console.log(fabFibonacci(7));
console.log(fabFibonacci(9));