//! Given an array and a value, count and print the number of array values greater than the provided value.

// Do-While Loop
const doWhile = (arr, val) => {
    let res = []
    let x = 0;
    do{
        arr[x] > val ? res.push(arr[x]) : '';
        x++;
    }
    while (x < arr.length);
    console.log(res);
}


// While Loop
const whileLoop = (arr, val) => {
    let res =[];
    let x = 0;
    while(x < arr.length){
        arr[x] > val ? res.push(arr[x]) : '';
        x++;
    }

    console.log(res);
}



// For Loop
const forLoop = (arr, val) => {
    let res = []
    for (let x = 0; x < arr.length; x++){
        arr[x] > val ? res.push(arr[x]) : ''
    }

    console.log(res);
}

const test = [5,6,1,14,15,13,7,8,12,2,3,4,9,10,11,16,17]
whileLoop(test, 6);