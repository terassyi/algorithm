# https://atcoder.jp/contests/typical90/tasks/typical90_x

n, k = list(map(int, input().split()))
a = list(map(int, input().split()))
b = list(map(int, input().split()))

diff = 0
for i in range(n):
	diff += abs(a[i] - b[i])

if diff == k:
	print("Yes")
elif diff < k:
	if (k - diff) % 2 == 0:
		print("Yes")
	else:
		print("No")
else:
	print("No")
