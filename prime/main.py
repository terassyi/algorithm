# 1000以下の素数

num = 1000

def seach(n):
    max = int(pow(n, 1/2))
    lis = [i for i in range(2, n+1)]
    prime = [2]
    for i in lis:
        isPrime = True
        for p in prime:
            if i % p == 0:
                isPrime = False
                break
        if isPrime:
            prime.append(i)
    return len(prime)

def main():
    print(seach(num))

if __name__ == '__main__':
    main()
