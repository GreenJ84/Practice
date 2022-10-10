// Given an array of numbers return a count of how many of the numbers are larger than the average.

function betterThanAverage(arr){
    var sum = 0;
    for (let i = 0; i < arr.length; i++){
        sum += arr[i]
    }
    sum = sum / arr.length;
    console.log(sum);
    var count = 0;
    for (let i = 0; i < arr.length; i++){
        if ( arr[i] > sum){
            count += 1
        }
    }
    return count;
}

const betterThanAverage2 = (arr) => {
    var sum = 0;
    arr.map(item => sum += item)
    sum = sum / arr.length;
    console.log(sum)
    var count = 0
    arr.map(item => item > sum ? count += 1 : '')
    return count;
}

var result = betterThanAverage2([6, 8, 3, 10, -2, 5, 9]);
console.log(result); // we expect back 4