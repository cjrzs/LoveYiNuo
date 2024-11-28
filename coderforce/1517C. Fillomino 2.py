n = int(input())
p = list(map(int, input().split(' ')))

res = []
for i in range(1, n + 1):
    res.append([-1] * i)

for i in range(n):
    cnt = v = p[i]
    a, b = i, i
    while cnt > 0:
        # print(a, b, cnt)
        if res[a][b] == -1:
            res[a][b] = v
            cnt -= 1
        if b - 1 >= 0 and res[a][b - 1] == -1:
            b -= 1
        else:
            a += 1
        # print(res)
for item in res:
    print(*item)








