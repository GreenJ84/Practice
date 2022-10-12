// Square each value in a given array, returning that same array with changed values.

function square(array){
    for (let i = 0; i < array.length; i++){
        let temp = array[i]
        array[i] = temp * temp;
    }
    console.log(array);
}

const square2 = (array) => {
    array = array.map(item => item*item);
    console.log(array);
}

square2([1,2,3,4,5]);
square2([8, 10, 12, 14]);
square2([10, 20, 30, 40 ,50, 60]);
square2([115]);