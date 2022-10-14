//Return the given array, after setting any negative values to zero.

function zeroOutNegatives(array){
    for(let i = 0; i < array.length; i++){
        if(array[i] < 0){
            array[i] = 0;
        }
    }
    return array;
}

const zeroOutNegatives2 = (array) => {
    array.map((idx) => {
        array[idx] < 0 ? array[idx] = 0 : '';
    })
    return array;
}
console.log(zeroOutNegatives2([1,-2,3,-4,5,6,7,-8,9]));