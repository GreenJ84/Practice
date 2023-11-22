# You are given two 0-indexed strings s and target. You can take some letters from s and rearrange them to form new strings.

# Return the maximum number of copies of target that can be formed by taking letters from s and rearranging them.

class Solution:
    def rearrangeCharacters(self, s: str, target: str) -> int:
        check = {char: 0 for char in set(target)}
        def track(char):
            if char in check:
                check[char] += 1
        list(map(lambda x: track(x), s))

        def checkEntries(entry):
            char, occ = entry
            count = int( occ / target.count(char) )
            return count

        return min(list(map(checkEntries, check.items() )))