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

function insertAt(idx, val, array){
    let i = idx + 1;
    let temp;
    let setLength = array.length;
    let set = array[idx];
    array[idx] = val;

    while (i < setLength){
        temp = array[i];
        array[i] = set;
        set = temp;
        i++;
    }
    array[i] = set;
    console.log(array)
    return array;
}

// function removeAt(){}


insertAt(3, 4, [1,2,3,5,6,7,8,9,10]);