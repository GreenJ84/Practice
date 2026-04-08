//If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//Create a function so that it returns the sum of all the multiples of 3 or 5 below the number passed in. Additionally, if the number is negative, return 0 (for languages that do have them).
var sum3and5toN = function (number) {
    var sum = 0;
    if (number < 0) {
        return sum;
    }
    var i = 3;
    while (i < number) {
        if (i % 3 === 0) {
            sum += i;
        }
        else if (i % 5 === 0) {
            sum += i;
        }
        i++;
    }
    return sum;
};
console.log(sum3and5toN(10));
