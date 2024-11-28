from collections import defaultdict


def dfs(u, f, cnt):
    if u in vis:
        return
    vis.add(u)
    if len(g[u]) == 1 and cnt > 1:
        path.append(cnt)
        return
    for i in g[u]:
        if i != f or f == -1:
            dfs(i, u, cnt + 1)


n, m = list(map(int, input().split(' ')))
g = defaultdict(list)
vis = set()
path = []
for _ in range(m):
    a, b = list(map(int, input().split(' ')))
    g[a].append(b)
    g[b].append(a)
# print(g)
for u in range(1, n + 1):
    dfs(u, -1, 1)
# print(path)
if len(path) == 0:
    print(0)
else:
    print(max(path) // 2)


