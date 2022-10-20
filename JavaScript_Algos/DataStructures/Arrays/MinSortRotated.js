// You will be given a numerical array that has first been sorted, then rotated by an unknown amount. Find and return the minimum value in that array.

function minSortedRotated(array){
    let min = array[0];

    for (let num in array){
        if (array[num] < min){
            min = array[num];
        }
    }
    console.log(min);
    return min;
}

minSortedRotated([7,8,9,5,6]);