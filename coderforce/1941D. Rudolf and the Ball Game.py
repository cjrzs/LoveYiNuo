import threading

import sys

# input = sys.stdin.readline
sys.setrecursionlimit(10 ** 8)


def func():
    res = []
    vis = set()

    def dfs(u, x):
        if u == m:
            res.append(x)
            return
        if (u, x) in vis:
            return
        vis.add((u, x))
        r, c = nums[u]
        r = int(r)
        if c == '0':
            dfs(u + 1, (x + n + r) % n)
        elif c == '1':
            dfs(u + 1, (x + n - r) % n)
        else:
            dfs(u + 1, (x + n + r) % n)
            dfs(u + 1, (x + n - r) % n)

    dfs(0, x)
    return res


T = int(input())

for _ in range(T):
    n, m, x = list(map(int, str(input()).split(' ')))
    nums = []
    for _ in range(m):
        r, c = str(input()).split(' ')
        nums.append((r, c))
    res = func()
    t = set()
    for item in res:
        if item == 0:
            item = n
        t.add(str(item))
    t = list(t)
    t.sort()
    print(len(t))
    print(' '.join(t))






