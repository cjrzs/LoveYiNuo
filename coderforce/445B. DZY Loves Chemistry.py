n, m = list(map(int, input().split(' ')))


p = [0] * (n + 1)
for i in range(1, n + 1):
    p[i] = i


def find(x):
    if p[x] != x:
        p[x] = find(p[x])
    return p[x]


def merge(a, b):
    global res
    u = find(a)
    v = find(b)
    if u != v:
        p[u] = v
        res *= 2


res = 1
for _ in range(m):
    a, b = list(map(int, input().split(' ')))
    merge(a, b)
print(res)





