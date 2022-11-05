class Solution:
    def pivotIndex(self, nums):
        if len(nums)==1:
            return 0
        rSum = sum(nums)-nums[0]
        lSum = 0
        if lSum == rSum:
            return 0
        for i in range(0,len(nums)):
            lSum+=nums[i]
            rSum-=nums[i+1]
            if lSum == rSum:
                return i+1
            if i+1 == len(nums)-1:
                return -1

s = Solution()
print(s.pivotIndex([1,7,3,6,5,6]))
print(s.pivotIndex([1,2,3]))
print(s.pivotIndex([2,1,-1]))