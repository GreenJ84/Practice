// Given an array, square each value in the array.

function square(arr){
    for ( let i = 0; i < arr.length; i++ ){
        arr[i] = arr[i]*arr[i];
    }
}

function square(arr){
    arr.map(item => item*item);
    return arr;
}