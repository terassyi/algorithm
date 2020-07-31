
def lengthOfLongestSubstring(self, s: str) -> int:
    # https://dev.classmethod.jp/articles/longest-substring-without-repeating-characters/
    window = set()
    size = 0
    i, j, n = 0, 0, len(s)
    while i < n and j < n:
        if s[j] not in window:
            window.add(s[j])
            j += 1
            size = max(len(window), size)
        else:
            window.discard(s[i])
            i += 1
    return size
