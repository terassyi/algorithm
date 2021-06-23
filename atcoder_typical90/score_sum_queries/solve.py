# https://atcoder.jp/contests/typical90/tasks/typical90_j

n = int(input())
c_p = [list(map(int, input().split())) for i in range(n)]
q = int(input())
l_r = [list(map(int, input().split())) for i in range(q)]

for l, r in l_r:
	a, b = 0, 0
	for i in range(l-1, r):
		c, p = c_p[i]
		if c == 1:
			a += p
		else:
			b += p
	print("{0} {1}".format(a, b))
