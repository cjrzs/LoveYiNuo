import sys
from collections import defaultdict, Counter, deque

# sys.setrecursionlimit(8 ** 10)

input = sys.stdin.readline


def input_nums():
    return list(map(int, input().split(' ')))

n, m = input_nums()
row = input()
col = input()
d = {'>': [0, 1], '<': [0, -1], 'v': [1, 0], '^': [-1, 0]}

def bfs(x, y):
    q = deque()
    q.append((x, y))
    vis = set()
    while q:
        a, b = q.popleft()
        vis.add((a, b))
        for k, val in d.items():
            i, j = a + val[0], b + val[1]
            if 0 <= i < n and 0 <= j < m and (k == col[b] if val[0] else k == row[a]) and (i, j) not in vis:
                q.append((i, j))
    # print(vis)
    return len(vis) == n * m


for i in range(n):
    for j in range(m):
        if not bfs(i, j):
            # print(g)
            print("NO")
            exit()
# print(g)
print("YES")




# def bfs(x, y):
#     q = [(x, y)]
#     t1 = set()
#     t1.add((x, y))
#     while q:
#         a, b = q.pop()
#         g[x][y].add((a, b))
#         if len(g[x][y]) == n * m:
#             return True
#         if row[a] == '>':
#             low, up = b + 1, m
#         else:
#             low, up = 0, b
#         for i in range(low, up):
#             if (a, i) not in g[x][y]:
#                 if g[a][i]:
#                     g[x][y] &= g[a][i]
#                     for k in g[a][i]:
#                         if k not in t1:
#                             q.append(k)
#                             t1.add(k)
#                 else:
#                     if (a, i) not in t1:
#                         q.append((a, i))
#                         t1.add((a, i))
#         if col[b] == 'v':
#             low, up = a + 1, n
#         else:
#             low, up = 0, a
#         for i in range(low, up):
#             if (i, b) not in g[x][y]:
#                 if g[i][b]:
#                     g[x][y] &= g[i][b]
#                     for k in g[i][b]:
#                         if k not in t1:
#                             q.append(k)
#                             t1.add(k)
#                 else:
#                     if (i, b) not in t1:
#                         q.append((i, b))
#                         t1.add((i, b))
#     return False





# from collections import deque
#
# dic = {'>': (0, 1), '<': (0, -1), 'v': (1, 0), '^': (-1, 0)}
# [n, m], hor, vert = map(int, input().split()), input().strip(), input().strip()
# for _x in range(n):
#     for _y in range(m):
#         q, d = deque([(_x, _y)]), [[0] * m for x in range(n)]
#         d[_x][_y] = 1
#         while q:
#             x, y = q.popleft()
#             for direction in '><v^':
#                 dx, dy = dic[direction]
#                 if -1 < x + dx < n and -1 < y + dy < m and (vert[y] == direction if dx else hor[x] == direction) and not \
#                 d[x + dx][y + dy]:
#                     d[x + dx][y + dy] = 1
#                     q.append((x + dx, y + dy))
#         if sum(sum(d, [])) != n * m:
#             print('NO')
#             break
#     else:
#         continue
#     break
# else:
#     print('YES')