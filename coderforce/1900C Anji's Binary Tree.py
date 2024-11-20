import sys
from collections import defaultdict, Counter
from typing import List


input = sys.stdin.readline


def dfs():
    q = [0]
    while q:
        u = q.pop()
        l = g[u][0]
        r = g[u][1]
        if l != -1:
            dist[l] = dist[u] + 1
            if s[u] == 'L':
                dist[l] = dist[u]
            q.append(l)
        if r != -1:
            dist[r] = dist[u] + 1
            if s[u] == 'R':
                dist[r] = dist[u]
            q.append(r)


T = int(input())
for _ in range(T):
    n = int(input())
    s = input()
    g = []
    vis = set()
    for i in range(n):
        x, y = list(map(int, input().split(' ')))
        if x == y == 0:
            vis.add(i)
        g.append([x - 1, y - 1])
    dist = [0] * n
    # print(g)
    dfs()
    # print(dist)
    res = float("inf")
    for i in range(n):
        if i in vis:
            res = min(res, dist[i])
    print(res)
