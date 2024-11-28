from collections import defaultdict


def dfs(start):
    q = [start]
    while q:
        u = q[-1]
        if u in vis:
            res[u] = 0
            q.pop()
            continue
        if u not in g:
            res[u] = cost[u]
            q.pop()
            continue
        t = True
        for x in g[u]:
            if res[x] == -1:
                q.append(x)
                t = False
        if t:
            res[u] = min(cost[u], sum([res[x] for x in g[u]]))
            q.pop()


for _ in range(int(input())):
    n, k = map(int, input().split(' '))
    cost = list(map(int, input().split(' ')))
    vis = set([x - 1 for x in map(int, input().strip().split(' '))])
    g = defaultdict(list)
    for i in range(n):
        t = input().strip()
        if t != '0':
            g[i].extend([x - 1 for x in map(int, t.split(' '))][1: ])
    # print(n, k, cost, vis, g)
    res = [-1] * n
    for i in range(n):
        if res[i] == -1:
            dfs(i)
    print(*res)








