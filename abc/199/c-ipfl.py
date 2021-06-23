
def main():
    n = int(input())
    s = list(input())
    q = int(input())
    queries = [ list(map(int, input().split())) for _ in range(q) ]
    s_1 = s[n:]
    s_2 = s[:n]
    reverse = False

    for q in queries:
        if q[0] == 1:
            a = q[1] - 1
            b = q[2] - 1
            if not reverse:
                if a < n and b < n:
                    s_1[a], s_1[b] = s_1[b], s_1[a]
                elif a < n and b >= n:
                    b = n - b
                    print("a = ", a)
                    print("b = ", b)
                    s_1[a], s_2[b] = s_2[b], s_1[a]
                else:
                    a = n - a
                    b = n - b
                    s_2[a], s_2[b] = s_2[b], s_2[a]
            else:
                if a < n and b < n:
                    # a = n - a
                    # b = n - b
                    print("rev s a = ", a)
                    print("rev s b = ", b)
                    s_2[a], s_2[b] = s_2[b], s_2[a]
                elif a < n and b >= n:
                    a = n - a
                    b = n - b
                    print("rev a = ", a)
                    print("rev b = ", b)
                    s_2[a], s_1[b] = s_1[b], s_2[a]
                else:
                    s_2[a], s_2[b] = s_2[b], s_2[a]
        else:
            print("rev")
            reverse = not reverse
    if reverse:
        print(''.join(s_2) + ''.join(s_1))
    else:
        print(''.join(s_1) + ''.join(s_2))



if __name__ == "__main__":
    main()

