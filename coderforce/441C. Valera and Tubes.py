from itertools import chain

n, m, k = map(int, input().split(' '))
x, y = 1, 1
cnt = 0
res = []
for _ in range(k - 1):
    t = []
    while len(t) < 2:
        cnt += 1
        t.append((x, y))
        if x % 2 == 1:
            if y == m:
                x += 1
            else:
                y += 1
        else:
            if y == 1:
                x += 1
            else:
                y -= 1
    res.append(t)


t = []
while len(t) < n * m - cnt:
    t.append((x, y))
    if x % 2 == 1:
        if y == m:
            x += 1
        else:
            y += 1
    else:
        if y == 1:
            x += 1
        else:
            y -= 1


res.append(t)
# print(res)

"""
res = [
    [(1, 1), (1, 2)], 
    [(1, 3), (2, 3)], 
    [(2, 2), (2, 1), (3, 1), (3, 2), (3, 3)]
]

2 1 1 1 2
2 1 3 2 3
5 2 2 2 1 3 1 3 2 3 3
"""

for x in res:
    # t = []
    # for z in x:
    #     t.append(z[0])
    #     t.append(z[1])
    # # print(len(x), *t)
    print(len(x), *[i for i in chain(*x)])

