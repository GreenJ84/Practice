// Implement a function ​sigma(num)​ that, given a number, returns the sum of all positive integers from 1 up to number (inclusive). Ex.: ​sigma(3) = 6 (or 1+2+3); ​sigma(5)​ = 15 (or 1+2+3+4+5)

function sigma(num){
    let sum = 0;
    while (num > 0){
        sum += num;
        num--
    }
    return sum
}

console.log(sigma(5));