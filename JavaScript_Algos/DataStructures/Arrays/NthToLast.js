// Return the element that is N-from-array’s-end.

function nthToLast(array, n){
    if (n <= array.length){
        return array[array.length-1-n];
    } 
    else{
        throw new Error("Out-Of-Bounds, No elements to be found")
    }
}

console.log(nthToLast([1,2,3,4,5,6,7,8,9,10], 0));
console.log(nthToLast([10,20,30,40,50,60,70,80,90,100], 1));
console.log(nthToLast([10,20,30,40,50,60,70,80,90,100], 9));
console.log(nthToLast([1,7,35,79,4,56,-2,19,88], 20));