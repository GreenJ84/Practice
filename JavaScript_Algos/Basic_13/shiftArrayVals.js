//? Given an array, move all values forward by one index, dropping the first and leaving a ​'0'​ value at the end.

function shiftArray(array){
    let counter = array[1]

    for (let i = 0; i < array.length; i++){
        array[i] = counter;
        counter = array[i+2]
    }
    array[array.length-1] = 0;
    return array
}

console.log(shiftArray([1,2,3,4,5,6,7,8,9]))