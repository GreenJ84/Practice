// Replace any negative array values with a string, 'No negaitves'​.

function noNegatives(array){
    if (array.length < 1) { return array }

    for (let i = 0; i < array.length; i++){
        if (array[i] < 0){
            array[i] = "No negatives";
        } 
    }
    console.log(array)

}


noNegatives([0,1,2,3,4,5,6,7,8,9,10]);
noNegatives([0,-1,2,-3,4,-5,6,7,8,9,-10]);