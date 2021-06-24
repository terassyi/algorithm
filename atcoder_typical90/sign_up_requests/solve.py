# https://atcoder.jp/contests/typical90/tasks/typical90_aa
n = int(input())
s = [input() for a in range(n)]

h = set()
for i in range(n):
	if s[i] in h:
		pass
	else:
		print(i+1)
		h.add(s[i])
