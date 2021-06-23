lis = list(map(int,input().split()))
a, b, c = lis[0], lis[1], lis[2]

if c % 2 is 0:
	if abs(a) < abs(b):
		print("<")
	elif abs(a) is abs(b):
		print("=")
	else:
		print(">")
else:
	if a < b:
		print("<")
	elif a is b:
		print("=")
	else:
		print(">")
