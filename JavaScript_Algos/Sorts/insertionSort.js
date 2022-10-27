// Best: O(n) linear when array is already sorted.
// Average: O(n^2) quadratic.
// Worst: O(n^2) quadratic when the array 
const numsOrdered = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const numsRandomOrder = [9, 2, 5, 6, 4, 3, 7, 10, 1, 8];
const numsReversed = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
const expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

//! Sorts the given array in-place.
const insertionSort = (nums) => {
    let runner = nums[1];
    // let j = i - 1;

    for (let i = 0; i < nums.length-1; i++){

        if (nums[i] > runner ){
            nums[i+1] = nums[i];
            nums[i] = runner;
        
            let j = i;
            let temp = nums[j];
            while (j > 0 && temp < nums[j-1]){
                    nums[j] = nums[j-1];
                    nums[j-1] = temp;
                    j--;
                    temp = nums[j];
            }
            i--;
        }
        runner = nums[i+2];
    }
}
insertionSort(numsReversed);
console.log(numsReversed);

insertionSort(numsRandomOrder);
console.log(numsRandomOrder);

let thhee = [9, 12, 2, 13, 5, 6, 15, 4, 14, 3, 7, 10, 1, 8, 11, 19, 18]
insertionSort(thhee);
console.log(thhee);
