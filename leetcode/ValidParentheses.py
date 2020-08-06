def isValid(s: str) -> bool:
    if len(s) % 2 != 0:
        return False
    openStack = []
    bracketDict = {"(": ")", "{": "}", "[": "]"}
    for c in s:
        if c == '(' or c == '{' or c == '[':
            openStack.append(c)
        else:
            if len(openStack) == 0:
                return False
            op = openStack.pop()
            if bracketDict[op] != c:
                return False
    if len(openStack) == 0:
        return True
    return False
