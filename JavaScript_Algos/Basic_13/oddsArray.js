//! Create an array with all the odd integers between 1 and 255 (inclusive).

// Do-While Loop
const doWhile = () => {
    let arr = [];
    let counter = 0;
    do{
        counter % 2 == 0 ? '' : arr.push(counter);
        counter++;
    }
    while(counter < 256);
    // console.log(arr);
    // console.log(arr[arr.length-1]);
}


// While Loop
const whileLoop = () => {
    let arr =[];
    let counter = 0;
    while(counter < 256){
        counter % 2 == 0 ? '' : arr.push(counter);
        counter++;
    }

    console.log(arr);
    console.log(arr[arr.length-1]);
}



// For Loop
const forLoop = () => {
    let arr = []
    for (let counter = 0; counter < 256; counter++){
        counter % 2 == 0 ? '' : arr.push(counter);
    }
    console.log(arr);
    console.log(arr[arr.length-1]);
}

