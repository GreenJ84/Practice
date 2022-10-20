// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
// A subarray is a contiguous part of an array.

function maxSubArray(array){
    let maxSum = array[0];
    let sum = 0;

    for (let num in array){
        sum += parseInt(array[num]);
        maxSum = Math.max(maxSum, sum);
        sum = Math.max(0, sum)
    }
    return maxSum;
}

// maxSubArray([-2,1,-3,4,-1,2,1,-5,4]);

var maxSubArray2 = function(array) {
    let sum = 0;
    let maxSum = array[0]
    for(let num of array){
        if(sum < 0) sum = 0;
        sum += num;
        maxSum = Math.max(maxSum, sum);
    }
    return maxSum;
};