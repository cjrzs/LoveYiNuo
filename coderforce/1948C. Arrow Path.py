from collections import defaultdict


def solve():
    q = [(0, 0)]
    vis = set()
    vis.add((0, 0))
    while q:
        x, y = q.pop()
        if x == 1 and y == n - 1:
            print("YES")
            return
        for a, b in g[(x, y)]:
            if (a, b) not in vis:
                q.append((a, b))
                vis.add((a, b))
    print("NO")


d = ([-1, 0], [0, 1], [1, 0], [0, -1])
T = int(input())
for _ in range(T):
    n = int(input())
    nums = []
    nums.append(list(input()))
    nums.append(list(input()))
    g = defaultdict(list)
    for i in range(2):
        for j in range(n):
            if (i + j) & 1:
                if nums[i][j] == '>':
                    if j + 1 < n:
                        g[(i, j)].append((i, j + 1))
                if nums[i][j] == '<':
                    if j - 1 >= 0:
                        g[(i, j)].append((i, j - 1))
            else:
                for x in d:
                    a, b = i + x[0], j + x[1]
                    if 0 <= a < 2 and 0 <= b < n:
                        g[(i, j)].append((a, b))
    del nums
    solve()


