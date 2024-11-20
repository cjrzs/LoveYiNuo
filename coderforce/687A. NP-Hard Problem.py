from collections import defaultdict


n, m = map(int, input().split(' '))
g = defaultdict(list)
for _ in range(m):
    a, b = map(int, input().split(' '))
    g[a].append(b)
    g[b].append(a)


vis = [0] * (n + 1)


for i in range(1, n + 1):
    if vis[i]: continue
    q = [i]
    vis[i] = 1
    while q:
        u = q.pop()
        for x in g[u]:
            if vis[x] == 0:
                vis[x] = 3 - vis[u]
                q.append(x)
            else:
                if vis[x] == vis[u]:
                    print("-1")
                    exit()

for i in [1, 2]:
    t = [j for j, x in enumerate(vis) if x == i]
    print(len(t))
    print(*t)
