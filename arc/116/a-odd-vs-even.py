# 正整数Nが与えられます
# Nの正の奇数の約数と正の偶数の約数はどちらが多いか答えてください。
# T個のテストケースが与えられるので、それぞれについて答えを求めてください。
ODD = "Odd"
EVEN = "Even"
SAME = "Same"

def main():
    t = int(input())
    n_list = []
    for _ in range(t):
        n_list.append(int(input()))
    for n in n_list:
        # if n is odd
        if n % 2 == 1:
            print(ODD)
        elif n % 4 == 0:
            print(EVEN)
        else:
            print(SAME)

if __name__ == "__main__":
    main()
