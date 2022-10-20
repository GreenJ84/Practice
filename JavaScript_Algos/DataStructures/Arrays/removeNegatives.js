// Implement a function ​removeNegatives()​ that accepts an array and removes any values that are less than zero.
// Second-level challenge: ​don’t use nested loops.

function removeNegatives(array){
    let i = 0;
    while (i < array.length){
        if (array[i] < 0){
            if ( i === array.length-1){ array.pop() }

            let j = i+1;
            while(j < array.length){
                array[j-1] = array[j];
                j++;
            }
            array.pop()
        }
        i++;
    }
    console.log(array);
    return array;
}
// removeNegatives([-1, 1, -2, 2, -3, 3]);


//? Pretty sure filter still uses loops under the hood but no explicit loops used
function removeNegatives2(array){
    return array.filter(item => item > -1);
}
// removeNegatives2([-1, 1, -2, 2, -3, 3]);
