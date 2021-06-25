# https://atcoder.jp/contests/typical90/tasks/typical90_g
from bisect import bisect_left

n = int(input())
a = list(map(int, input().split()))
q = int(input())
b = [int(input()) for i in range(q)]

# for i in b:
	# m = abs(a[0] - i)
	# for j in a:
		# if abs(j - i) < m:
			# m = abs(j - i)
	# print(m)

a.sort()

# def bin_search(l, val: int) -> int:
	# # print(l)
	# if len(l) == 1:
		# return abs(l[0] - val)
	# elif len(l) == 2:
		# if abs(l[0] - val) < abs(l[1] - val):
			# return abs(l[0] - val)
		# else:
			# return abs(l[1] - val)
	# m = len(l) // 2
	# if abs(l[m-1] - val) > abs(l[m] - val):
		# return bin_search(l[m:], val)
	# else:
		# return bin_search(l[:m], val)

# for i in b:
	# val = bin_search(a, i)
	# print(val)
	# # print(abs(i - val))

for i in b:
	idx = bisect_left(a, i)
	if idx == 0:
		print(a[0] - i)
	elif idx == n:
		print(i - a[-1])
	else:
		print(min(i - a[idx - 1], a[idx] - i))
