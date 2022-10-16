// Modern computers can handle very large numbers, but this ability has limits. If a number is repeatedly multiplied by itself, it eventually exceeds the computer’s ability to accurately represent it. (Side note: the number of times it is multiplied by itself is called the ​exponent​.) For an optional end-of-chapter challenge, determine the smallest (least significant) digit of a number that is potentially very, very large. You may find that you must do this without computing the actual (unimaginably large) number.
// Implement a function that accepts two non-negative integers as arguments. Function ​lastDigitAtoB(a, b)​ should return the last digit of the first number (a) raised to an exponent of the second number (b).
// Examples: given ​(3, 4)​, you should return ​1​ (the last digit of 81: 3 * 3 * 3 * 3). Given ​(12, 5)​, return 2​ (the least significant digit of 248832, which is 12 * 12 * 12 * 12 * 12).

function lastDigitAtoB(a, b){
    if (a < 0 || b < 0){
        throw Error('Numbers need to be positive!')
    }
    let sum = 1;
    while (b > 0){
        sum *= a;
        b--;
    }
    return sum % 10;
}

console.log(lastDigitAtoB(3,4));
console.log(lastDigitAtoB(12, 5));
// console.log(lastDigitAtoB(-2,-1));