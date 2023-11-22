class Solution:
    def runningSum(self, nums):
        sum = nums[0]
        for i in range(0, len(nums)):
            nums[i] = sum
            if i != len(nums)-1:
                sum += nums[i+1]
            i+=1
        return nums

s = Solution()
print(s.runningSum([1,2,3,4]))
print(s.runningSum([1,1,1,1,1]))
print(s.runningSum([3,1,2,10,1]))
