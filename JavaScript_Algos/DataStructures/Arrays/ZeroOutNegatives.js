// Given an array, set negative values to zero.

function zeroOut(arr){
    for ( let i = 0; i < arr.length; i++){
        if ( arr[i] < 0 ){
            arr[i] = 0;
        }
    }
    return arr;
}

console.log(zeroOut([-1,1,-2,2,-3,3,-4,4,-5,5,-6,6]));