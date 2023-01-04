class Solution:
    def reverseBits(self, n: int) -> int:
        res = 0
        for i in range(32):
            bit = (n >> i) & 1
            res = res | (bit << (31 - i))
        return res

s = Solution()
print(s.reverseBits(43261596))
print(s.reverseBits(4294967293))