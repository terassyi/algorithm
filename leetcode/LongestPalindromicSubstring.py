def longestPalindrome(s: str) -> str:
    # Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
    # Input: "babad"
    # Output: "bab"
    # Note: "aba" is also a valid answer.
    # https://en.wikipedia.org/wiki/Longest_palindromic_substring#:~:text=In%20computer%20science%2C%20the%20longest,bananas%22%20is%20%22anana%22.
    # https://qiita.com/Shawna/items/17ed870cd4c8c0f1f478
    if len(s) < 2:
        return s
    start = 0
    maxLen = 1
    i = 0

    while i < len(s):
        l = i
        r = i
        while r < len(s) - 1 and s[r] == s[r+1]:
            r += 1
        i = r + 1
        while r < len(s)-1 and l > 0 and s[r+1] == s[l-1]:
            l -= 1
            r += 1
        if maxLen < r - l + 1:
            start = l
            maxLen = r - l + 1
    return s[start: start + maxLen]
    
print(longestPalindrome("babad"))
