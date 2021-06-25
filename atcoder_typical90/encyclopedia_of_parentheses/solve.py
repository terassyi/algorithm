# https://atcoder.jp/contests/typical90/tasks/typical90_b

# s -> (s)|st|ts
n = int(input())

if n % 2 == 1:
	print()
	exit(0)

pairs = [["", 0]]
for i in range(n):
	t = []
	for s, d in pairs:
		if d == 0:
			t.append([s+"(", d+1])
		else:
			t.append([s+"(", d+1])
			t.append([s+")", d-1])
	pairs = t

ans = []
for s, d in pairs:
	if d == 0:
		ans.append(s)

ans.sort()
for a in ans:
	print(a)
