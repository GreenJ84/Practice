// Implement ​rotateArr(arr, shiftBy)​ that accepts array and offset. Shift arr’s values ​to the right​ by that amount. ‘Wrap-around’ any values that shift off array’s end to the other side, so that no data is lost. Operate in-place: given ([1,2,3],1)​, change the array to [​ 3,1,2]​.
// Optionally, add these advanced features:
// a) allow a negative shiftBy (shift left, not right),
// b) minimize memory usage. With only a few local variables (not an array), handle arrays and shiftBys in the millions,
// c) minimize how many touches of each element.

function rotateArr(array, offset){
    let result = new Array(array.length);
    offset = offset % array.length;
    let i = 0;

    while ( i < array.length ){
        result[(i + offset) % array.length] = array[i];
        i++;
    }
    console.log(result);
    return result;
}

rotateArr([1,2,3], 2);

