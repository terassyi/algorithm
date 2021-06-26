# https://atcoder.jp/contests/typical90/tasks/typical90_n

n = int(input())
a = list(map(int, input().split()))
b = list(map(int, input().split()))

a.sort()
b.sort()

# table = []
# for i in b:
	# t = []
	# for j in a:
		# t.append(abs(i-j))
	# table.append(t)

# ans = 0
# for i in range(n):
	# ans += table[i][i]
ans = 0
for i in range(n):
	ans += abs(a[i] - b[i])

print(ans)
