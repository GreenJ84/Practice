# Given an array of characters chars, compress it using the following algorithm:
# Begin with an empty string s. For each group of consecutive repeating characters in chars:
# If the group's length is 1, append the character to s.
# Otherwise, append the character followed by the group's length.
# The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.
# After you are done modifying the input array, return the new length of the array.
# You must write an algorithm that uses only constant extra space.

from typing import List


class Solution:
    def compress(self, chars: List[str]) -> int:
        n = len(chars)
        if n <= 1:
            return n
        
        write_idx = 0
        count = 1
        
        for i in range(1, n):
            if chars[i] == chars[i - 1]:
                count += 1
            else:
                chars[write_idx] = chars[i - 1]
                write_idx += 1
                if count > 1:
                    add = str(count)
                    for char in add:
                        chars[write_idx] = char
                        write_idx += 1
                    count = 1

        chars[write_idx] = chars[n - 1]
        write_idx += 1
        if count > 1:
                    add = str(count)
                    for char in add:
                        chars[write_idx] = char
                        write_idx += 1
        return write_idx


s = Solution();
print(s.compress(["a", "a", "b", "b", "c", "c", "c"]))
assert s.compress(["a", "a", "b", "b", "c", "c", "c"]) == 6

print(s.compress(["a"]))
assert s.compress(["a"]) == 1

print(s.compress(["a","b","b","b","b","b","b","b","b","b","b","b","b"]))
assert s.compress(["a","b","b","b","b","b","b","b","b","b","b","b","b"]) == 4