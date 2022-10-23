// Return the second-largest element of an array.

function secondLargest(array){
    let largest = 0;
    let secondLargest = 0;

    for (let i = 0; i < array.length; i++){
        if (array[i] > largest){
            secondLargest = largest;
            largest = array[i];
        } else if (array[i] > secondLargest && array[i] < largest){
            secondLargest = array[i];
        }
    }
    return secondLargest;
};

console.log(secondLargest([1,2,3,4,5,6,7,8,9]));
console.log(secondLargest([10,20,30,40,50,60,70,80,90]));
console.log(secondLargest([10, 14, 19, 28, 89, 72 ,56, 43, 20, 88, 01, 79]));