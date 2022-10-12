// Iterate through a given array, printing each value.

function printArrayValues(array){
    for (let i = 0; i < array.length; i++){
        console.log(array[i]);
    }
}
function printArrayValues2(array){
    let i = 0
    while (i < array.length){
        console.log(array[i]);
        i++;
    }
}

const printArrayValues3 = (array) => {
    array.map(item => console.log(item));
}

printArrayValues3([0,1,2,3,4,5,6,7,8,9,10]);
printArrayValues3([6,2,4,7,9,8,1,2,10]);