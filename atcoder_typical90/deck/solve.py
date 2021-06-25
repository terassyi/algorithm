# https://atcoder.jp/contests/typical90/tasks/typical90_bi

q = int(input())
t_x = [list(map(int, input().split())) for i in range(q)]

deck = []
for t, x in t_x:
	if t == 1:
		deck.insert(0, x)
	elif t == 2:
		deck.append(x)
	else:
		print(deck[x-1])
