import sys

sys.setrecursionlimit(15000)
n = int(input())
se1 = set()
se2 = set()
a, b = map(int, input().split())
x, y = map(int, input().split())
lis = []
for i in range(n):
    lis.append(input())


def dfs(x, y, f):
    if x < 0 or y < 0 or x >= len(lis) or y >= len(lis[0]) or lis[x][y] == '1':
        return
    if f == 0:
        if (x, y) in se1:
            return

        se1.add((x, y))
    else:
        if (x, y) in se2:
            return
        se2.add((x, y))
    dfs(x - 1, y, f)
    dfs(x + 1, y, f)
    dfs(x, y - 1, f)
    dfs(x, y + 1, f)


dfs(a - 1, b - 1, 0)
dfs(x - 1, y - 1, 1)
ans = float('Inf')

for i in se1:
    if i in se2:
        print(0)
        exit()
    for j in se2:
        ans = min(ans, (i[0] - j[0]) ** 2 + (i[1] - j[1]) ** 2)
print(ans)
