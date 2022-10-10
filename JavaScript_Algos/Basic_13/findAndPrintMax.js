// Given an array of numbers find and printe the largest element within the array

// Finds largest integer within an integer array
const findMaxInt = (array) => {
    let max = 0;

    for (let i = 0; i < array.length; i++){
        if (array[i] > array[max]){
            max = i;
            console.log(max);
        }
    }
    console.log(array[max]);
    return array[max];
}

//Finds the longest String withing an array of Strings
const findMaxString = (array) => {
    let max = 0;

    for (let i = 0; i < array.length; i++){
        if (array[i].length > array[max].length){
            max = i;
            console.log(max);
        }
    }
    console.log(array[max]);
    return array[max];
}
