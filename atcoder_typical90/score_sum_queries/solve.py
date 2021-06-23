# https://atcoder.jp/contests/typical90/tasks/typical90_j

n = int(input())
c_p = [list(map(int, input().split())) for i in range(n)]
q = int(input())
l_r = [list(map(int, input().split())) for i in range(q)]

a_b = []
a, b = 0, 0
for c, p in c_p:
	if c == 1:
		a += p
	else:
		b += p
	a_b.append([a, b])

for l, r in l_r:
	if l == 1:
		print("{0} {1}".format(a_b[r-1][0], a_b[r-1][1]))
		continue
	a = a_b[r-1][0] - a_b[l-2][0]
	b = a_b[r-1][1] - a_b[l-2][1]
	print("{0} {1}".format(a, b))
