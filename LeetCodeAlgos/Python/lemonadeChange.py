# At a lemonade stand, each lemonade costs $5. Customers are standing in a queue to buy from you and order one at a time (in the order specified by bills). Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill. You must provide the correct change to each customer so that the net transaction is that the customer pays $5.
# Note that you do not have any change in hand at first.
# Given an integer array bills where bills[i] is the bill the ith customer pays, return true if you can provide every customer with the correct change, or false otherwise.

from typing import List


class Solution:
    def lemonadeChange(self, bills: List[int]) -> bool:
        change = [0, 0, 0]
        for i in bills:
            match i:
                case 5:
                    change[0] += 1
                case 10:
                    change[0] -= 1
                    change[1] += 1
                case 20:
                    if change[1] >= 1:
                        change[0] -= 1
                        change[1] -= 1
                    else:
                        change[0] -= 3

            if change[0] < 0 or change[1] < 0: 
                return False
        return True
    
s = Solution()
print(s.lemonadeChange([5,5,5,10,20]))
print(s.lemonadeChange([5,5,10,10,20]))