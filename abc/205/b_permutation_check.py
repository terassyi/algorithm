n = int(input())
lis = list(map(int,input().split()))

if len(lis) is 1 and lis[0] is 1:
	print("Yes")
	exit(0)

lis.sort()
if lis[0] is not 1 or lis[n-1] is not n: 
	print("No")
	exit(0)

for i in range(n-1):
	if lis[i] is lis[i+1]:
		print("No")
		exit(0)

if lis[n-1] is lis[n-2]:
	print("No")
	exit(0)

print("Yes")
