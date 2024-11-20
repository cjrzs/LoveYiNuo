import sys
sys.setrecursionlimit(8 ** 10)

from collections import defaultdict


g = defaultdict(list)
n = int(input())
for _ in range(n - 1):
    a, b = map(int, input().split(' '))
    g[a].append(b)
    g[b].append(a)

cnt = [0] * 2

q = [(1, 1)]
color = 1
vis = set()
vis.add(1)
while q:
    cur, color = q.pop()
    cnt[color] += 1
    for x in g[cur]:
        if x not in vis:
            q.append((x, 1 - color))
            vis.add(x)

print(cnt[0] * cnt[1] - (n - 1))

