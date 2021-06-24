# https://atcoder.jp/contests/typical90/tasks/typical90_bc

n, p, q = list(map(int, input().split()))
a = list(map(int, input().split()))
ans = 0

for i in range(n):
	for j in range(i+1, n):
		for k in range(j+1, n):
			for l in range(k+1, n):
				for m in range(l+1, n):
					val = (((a[i] * a[j] % p) * a[k] % p) * a[l] % p) * a[m] % p
					if val == q:
						ans += 1

print(ans)
