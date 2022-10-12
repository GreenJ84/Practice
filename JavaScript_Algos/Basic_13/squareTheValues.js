// Square each value in a given array, returning that same array with changed values.

function square(array){
    for (let i = 0; i < array.length; i++){
        let temp = array[i]
        array[i] = temp * temp;
    }
    console.log(array);
}

square([1,2,3,4,5]);
square([8, 10, 12, 14]);
square([10, 20, 30, 40 ,50, 60]);
square([115]);