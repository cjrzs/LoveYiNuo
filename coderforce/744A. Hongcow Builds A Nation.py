def find(x):
    if p[x] != x:
        p[x] = find(p[x])
    return p[x]


def merge(a, b):
    x, y = find(a), find(b)
    # print(a, x, b, y)
    if x != y:
        p[x] = y
        edges[y] += edges[x]
        edges[x] = 0
        cnt[y] += cnt[x]
        cnt[x] = 0
        if police[x] == 1:
            police[y] = 1
            police[x] = 0
    # print(edges)


n, m, k = map(int, input().split(' '))
vis = set(map(int, input().split(' ')))
edges = [0] * (n + 1)
p = list(range(n + 1))
cnt = [1] * (n + 1)
police = [0] * (n + 1)
for i in vis:
    police[i] = 1
nums = []
for i in range(m):
    a, b = map(int, input().split(' '))
    edges[a] += 1
    nums.append((a, b))

for a, b in nums:
    merge(a, b)

# print(p, cnt, edges, police)
res = 0
total = 1
summ = 0
maxx = max([cnt[find(x)] for x in range(1, n + 1) if police[find(x)] == 1])
for i in range(1, n + 1):
    t = find(i)
    if t == i:
        # print(t)
        res += (cnt[t] ** 2 - cnt[t]) // 2 - edges[t]
for i in range(1, n + 1):
    t = find(i)
    if t == i and police[t] == 0:
        res += cnt[t] * maxx
        maxx += cnt[t]

# print("--", maxx, total)
print(res)






