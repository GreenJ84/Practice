// Recreate the ​shuffle()​built into JavaScript, to efficiently shuffle a given array’s values. Return the array from your function.

function shuffle( array ){
    let times = 0;
    let right = true;
    let i = 0;
    while( i < array.length && i >= 0){
        let to = Math.floor(Math.random() * array.length)
        let from = Math.floor(Math.random() * array.length)
        let temp = array[to];
        array[to] = array[from];
        array[from] = temp;

        if (i === array.length){
            right = false;
            times += 1;
        } else if( i = -1){
            right = true;
            times += 1;
        }
        if (times >= 100){
            break;
        }
        if (right === true ){
            i++;
        } else { i--;}
    }
    return array;
}

console.log(shuffle([1,2,3,4,5,6,7,8,9]));
console.log(shuffle([10,20,30,40,50,60,70,80,90]));
console.log(shuffle([10, 89, 24, 56, 32, 43, 66]));