# https://atcoder.jp/contests/typical90/tasks/typical90_p

n = int(input())
a, b, c = list(map(int, input().split()))

ans = 10 ** 9
for i in range(10 ** 4): 
    for j in range(10 ** 4): 
        k = n - (a * i + b * j)
        if k % c == 0 and k >= 0:
            ans = min(ans, i + j + k // c)
print(ans)
