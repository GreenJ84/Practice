// Use a time-space tradeoff to accelerate the average running time of an iSigma(num) function that returns the sum of all positive integers from 1 to num. Recall: sig(1) = 1, sig(2) = 3, sig(3) = 6, sig(4) = 10.


function smarterSum(n){
    let result = 0;
    for (let i in [...Array(n+1).keys()]){
        result += parseInt(i);
    }
    return result;
}

console.log(smarterSum(1));
console.log(smarterSum(3));
console.log(smarterSum(5));
console.log(smarterSum(10));