//Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number.
// Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
const twoSum = (self, numbers, target) => {

        start = 0
        end = len(numbers)-1
        if (numbers.length < 2){
            return null
        }
        
        while (start < end_){ 
            sum = numbers[start] + numbers[end]
            print(sum)
            if (sum > target){
                end -= 1
            }
            else if(sum < target){
                start += 1
            }
            else{
                return [start+1, end+1]
            }
        }
}