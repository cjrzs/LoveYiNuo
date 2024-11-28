def find(x):
    if p[x] != x:
        p[x] = find(p[x])
    return p[x]


n, m = map(int, input().split(' '))
flag = False
cnt = 0
p = [0] * (n + 1)
for i in range(1, n + 1):
    p[i] = i
for _ in range(m):
    a, b = map(int, input().split(' '))
    x, y = find(a), find(b)
    if x != y:
        p[x] = y
    else:
        if not flag and cnt == 0:
            flag = True
            cnt += 1
        else:
            flag = False
if n < 3 or n != m:
    print("NO")
else:
    print("FHTAGN!" if flag else "NO")



