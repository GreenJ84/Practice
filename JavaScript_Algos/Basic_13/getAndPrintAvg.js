// Analyze an array’s values and print the average

function printAvg(array){
    let sum = 0;
    for (let i = 0; i < array.length; i++){
        sum += array[i];
    }
    console.log(sum / array.length)
}


printAvg([0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10,0,1,2,3,4,5,6,7,8,9,10]);
printAvg([2,4,6,8,10]);