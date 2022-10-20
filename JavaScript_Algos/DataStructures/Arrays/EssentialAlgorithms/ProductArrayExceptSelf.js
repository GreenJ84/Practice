// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
// You must write an algorithm that runs in O(n) time and without using the division operation.

var productExceptSelf1 = function(nums) {
    let check = [...nums]
    check = check.reduce((a,b) => a*b);

    let results = [];
    let i = 0;
    while ( i < nums.length){
        if (nums[i] != 0){
            results.push(check/nums[i]);
        }
        else {
            let temp = 1, start = 0, end = nums.length-1;
            while (start < end){
                if (start != i){
                    temp *= nums[start];
                    start++;
                } else if (end != i){
                    temp *= nums[end];
                    end--;
                }
            }
            results.push(temp);
        }
        i++;
    }
    console.log({results})
    return results
};


function settleZero(i, nums){
    let temp = 1, start = 0, end = nums.length-1;
            while (start < end){
                if (start != i){
                    temp *= nums[start];
                    start++;
                } else if (end != i){
                    temp *= nums[end];
                    end--;
                }
            }
        return temp;
}
var productExceptSelf2 = function(nums) {
    let product = 1, start = 0, end = nums.length-1;

    while (start < end){
        product *= nums[start];
        start++;
        product *= nums[end];
        end--;
    }
    if (start === end){ product *= nums[start] }

    let results = [];
    let i = 0;
    while ( i < nums.length){
        nums[i] !== 0 ? results.push(product/nums[i]) : results.push(settleZero(i, nums));

        if (i+1 != nums.length){
            nums[i+1] !== 0 ? results.push(product/nums[i+1]) : results.push(settleZero(i+1, nums))
        }
        i += 2;
    }
    console.log({results})
    return results
};


// productExceptSelf1([1,2,3,4]);
// productExceptSelf2([-1,1,0,-3,3]);
productExceptSelf2([1,0]);
