# https://atcoder.jp/contests/typical90/tasks/typical90_bo

l = input().split()
k = int(l[1])

def base_8_to_10(num):
	a = len(num)
	res = 0
	for i in range(a):
		n = int(num[a-i-1])
		res += n * (8 ** i)
	return res

def base_10_to_9(num):
	s = ""
	while True:
		if num < 9:
			if num == 8:
				num = 5
			s = str(num) + s
			break
		a = num % 9
		if a == 8:
			a = 5
		num = num // 9
		s = str(a) + s
	return int(s)

r = l[0]
for i in range(k):
	a = base_8_to_10(r)
	r = str(base_10_to_9(a))
	
print(r)
