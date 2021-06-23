# https://atcoder.jp/contests/typical90/tasks/typical90_d

h, w = map(int, input().split())
m = [list(map(int,input().split())) for i in range(h)]

sum_c = [sum(l) for l in m]
sum_r = []
for i in range(w):
	tmp = 0
	for j in range(h):
		# print("{0} {1}".format(i, j))
		tmp += m[j][i]
	sum_r.append(tmp)
ans = []
for i in range(h):
	tmp = []
	for j in range(w):
		a = sum_c[i] + sum_r[j] - m[i][j]
		tmp.append(a)
	ans.append(tmp)

for l in ans:
	for i in l:
		print(i, end=' ')
	print()
