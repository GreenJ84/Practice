// All Things Array Manipulation
// Basic manipulations of Array types

function pushToFront(val, array){
    let i = 0;
    let temp = 0;
    let set = array[i];
    let setLength = array.length;

    while( i < setLength ){
        temp = array[i+1];
        array[i+1] = set;
        set = temp;
        i++
    }
    array[0] = val;
    console.log(array);
    return array;
}

function popFront(array){
    let i = 1;
    while (i < array.length){
        array[i-1] = array[i];
        i++;
    }
    array.pop();
    console.log(array);
    return array;
}

// function insertAt(){

// }

// function removeAt(){}


popFront([1,2,3,4,5,6,7,8,9,10]);