// Given an array, return the Nth-largest element: there should be (N - 1) elements that are larger.

function nthLargest(array, n){
    array.sort()
    array.reverse()
    console.log(array);
    return array[n-1];
}


console.log(nthLargest([1,2,3,4,5,6,7,8,9], 6));
console.log(nthLargest([10,20,30,40,50,60,70,80,90], 3));
console.log(nthLargest([10, 14, 19, 28, 89, 72 ,56, 43, 20, 88, 01, 79], 1));