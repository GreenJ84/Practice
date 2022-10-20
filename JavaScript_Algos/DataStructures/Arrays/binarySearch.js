// Given a sorted array and a value, return whether that value is present in the array. Do not sequentially iterate the entire array. Instead, ‘divide and conquer’, taking advantage of the fact that the array is sorted.

function binarySearch(array, target){
    let i = Math.round(array.length/2);
    let start = 0;
    let end = array.length-1;
    while (start < i && i < end){
        let check = array[i];
        if ( check === target ){
            return true;
        }
        else if ( check > target){
            end = i;
            let temp = Math.round(start + (i - start)/2);
            i = temp;
        }
        else if ( check < target ){
            start = i;
            let temp = Math.round(end - (end - i)/2);
            i = temp
        }
    }
    return false;
}

console.log(binarySearch([1,2,4,5,6,7,8,9,10], 3));