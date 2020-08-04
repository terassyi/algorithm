def longestCommonPrefix(strs: List[str]) -> str:
    # Input: ["flower","flow","flight"]
    # Output: "fl"
    ans = ""
    minLen = 0
    for s in strs:
        if minLen > len(s):
            minLen = len(s)
    
    for index in range(minLen):
        c = strs[0][index]
        for s in strs:
            if c != s[index]:
                return ans
            
        ans = ans + c
    return ans
