# Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

# I can be placed before V (5) and X (10) to make 4 and 9. 
# X can be placed before L (50) and C (100) to make 40 and 90. 
# C can be placed before D (500) and M (1000) to make 400 and 900.
# Given an integer, convert it to a roman numeral.

class Solution:
    def intToRoman(self, num: int) -> str:
        roman = {
            1000: 'M',
            900: 'CM',
            500: 'D',
            400: 'CD',
            100: 'C',
            90: 'XC',
            50: 'L',
            40: 'XL',
            10: 'X',
            9: 'IX',
            5: 'V',
            4: 'IV',
            1: 'I'
        }
        result = ''
        
        for key, value in roman.items():
            while num >= key:
                result += value
                num -= key
                
        return result

s = Solution()
print(s.intToRoman(3))
assert s.intToRoman(3) == "III"

print(s.intToRoman(58))
assert s.intToRoman(58) == "LVIII"

print(s.intToRoman(1994))
assert s.intToRoman(1994) == "MCMXCIV"

print(s.intToRoman(3999))
assert s.intToRoman(3999) == "MMMCMXCIX"