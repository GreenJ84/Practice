// Write a function that given a sorted array of page numbers, return a string representing a book index. Combine consecutive pages to create ranges. Given ​[1, 3, 4, 5, 7, 8, 10]​, return the string ​"1, 3-5, 7-8, 10"​.

function bookIndex(arr){
    let result = '';
    for( let i = 0; i < arr.length; i++){
        let span = false;
        result += arr[i];
        if (arr[i]+1 === arr[i+1]){ result += '-'; span = true }
        while ( arr[i]+1 === arr[i+1] ){
            i++
        }
        if ( span ){ result += arr[i] }
        if ( i != arr.length-1 ){ result += ', ' }
    }
    console.log(result);
    return result;
}

bookIndex([1, 3, 4, 5, 7, 8, 10]);
bookIndex([10, 31,32,33,34,35, 41, 46,47,48,49, 55, 67, 68, 69,  100]);