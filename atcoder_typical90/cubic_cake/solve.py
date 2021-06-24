# https://atcoder.jp/contests/typical90/tasks/typical90_v
import functools
import math

lis = list(map(int, input().split()))

e = functools.reduce(math.gcd, lis) 
ans = (lis[0] // e) + (lis[1] // e) + (lis[2] // e) - 3
print(ans)
