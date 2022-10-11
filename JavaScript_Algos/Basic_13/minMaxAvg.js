// Given an array, print the max, min and average values for that array.

function minMaxAvg(array){
    if (array.length < 1) { return null }

    let max = array[0];
    let min = array[0];
    let avg = 0;

    for (let i = 0; i < array.length; i++){
        if (array[i] < min){
            min = array[i];
        } else if (array[i] > max){
            max = array[i];
        }
        avg += array[i];
    }
    avg /= array.length;
    console.log('The array min is '+min+', the max is '+max+', and the avg is '+avg+".")

}

minMaxAvg([0,1,2,3,4,5,6,7,8,9,10]);