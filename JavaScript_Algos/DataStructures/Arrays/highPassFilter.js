// Given an array and a value cutoff, return a new array containing only the values larger than cutoff.

function highPass(arr, cutoff) {
    var filteredArr = [];
    for (item in arr){
        if (arr[item] > cutoff){
            filteredArr.push(arr[item]);
        }
    }
    return filteredArr;
}

const highPass2 = (arr, cutoff) => {
    var filteredArr = [];
    arr.map(item => {item > cutoff ? filteredArr.push(item) : ''})
    return filteredArr;
}

var result = highPass2([6, 8, 3, 10, -2, 5, 9], 5);
console.log(result); // we expect back [6, 8, 10, 9]