// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

var twoSum = function(nums, target) {
    let results = []
    let check = [...nums];
    check.sort((a, b) => a - b);
    
    if (check.length < 1){ return results }
    if (check.length === 2 && check[0] + check[1] != target){ return results }
    
    let start = 0;
    let end = check.length-1;
    while(start < end){
        if(check[start] + check[end] == target){
            if( check[start] === check[end]){
            results.push(nums.indexOf(check[start]));
            nums.splice(nums.indexOf(check[start]), 1);
            results.push(nums.indexOf(check[end])+1);
            }
            else {
            results = [nums.indexOf(check[start]), nums.indexOf(check[end])];
            }
            return results
        } 
        else if(check[start] + check[end] > target){
            end--
        } 
        else if(check[start] + check[end] < target){
            start++
        }
    }
    return results
};

// console.log(twoSum([3,2,4], 6));
console.log(twoSum([3,3], 6));
