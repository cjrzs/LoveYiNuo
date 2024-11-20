T = int(input())
res = []
for _ in range(T):
    a, b, c = list(map(int, input().split(' ')))
    if b == 0:
        if a == 0:
            res.append('1' * (c + 1))
        elif c == 0:
            res.append('0' * (a + 1))
    else:
        t = ''
        t += '0' * (a + 1)
        t += '1' * (c + 1)
        for i in range(b - 1):
            if i % 2:
                t += '1'
            else:
                t += '0'
        res.append(t)
    # if b == 0:
    #     if a == 0:
    #         res.append('1' * (c + 1))
    #     elif c == 0:
    #         res.append('0' * (a + 1))
    #     continue
    # elif a == 0:
    #     t = '1' * (c + 1)
    #     if c == 0:
    #         res.append('01' * b)
    #         continue
    #     if b % 2 == 1:
    #         t += '01' * (b // 2) + '0'
    #     else:
    #         t += '01' * (b // 2)
    #     res.append('0' * (a + 1))
    #     continue
    # elif c == 0:
    #     t = '0' * (a + 1)
    #     if b % 2 == 1:
    #         t += '10' * (b // 2) + '1'
    #     else:
    #         t += '10' * (b // 2)
    #     res.append('0' * (a + 1))
    #     continue
    # else:
    #     t = '1' * (c + 1)
    #     t += '0' * (a + 1)
    #     t += '10' * ((b - 2) // 2 + 1)
    #     res.append(t)
    #     continue

for x in res:
    print(x)











