for _ in range(int(input())):
    n, k = map(int, input().split(' '))
    s = input()
    res = 0
    for i in range(k):
        b = [0] * 26
        for j in range(i, n, k):
            t1 = ord(s[j]) - 97
            b[t1] += 1
            t2 = ord(s[k - j - 1]) - 97
            b[t2] += 1
        res += sum(b) - max(b)
    print(res // 2)




