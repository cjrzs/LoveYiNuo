import sys
from collections import deque

sys.setrecursionlimit(8 ** 10)

n, m = list(map(int, input().split(' ')))


def bfs(u):
    q = deque()
    q.append((u, 0))
    vis = set()

    while q:
        cur, level = q.popleft()
        vis.add(cur)
        if cur == m:
            return level
        t1 = cur - 1
        t2 = cur * 2
        if t1 >= 1 and t1 not in vis:
            q.append((t1, level + 1))
        if t2 <= m * 2 and t2 not in vis:
            q.append((t2, level + 1))
    return 0


print(bfs(n))








