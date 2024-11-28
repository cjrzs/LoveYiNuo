import heapq
import sys
import threading
from collections import defaultdict, Counter, deque
from itertools import permutations, chain, combinations
from types import GeneratorType

from typing import List, Set, Dict, Tuple, AnyStr

input = sys.stdin.readline
MOD = 10 ** 9 + 7


def bootstrap(f, stack=[]):
    def wrapped_func(*args, **kwargs):
        if stack:
            return f(*args, **kwargs)
        else:
            to = f(*args, **kwargs)
            while True:
                if type(to) is GeneratorType:
                    stack.append(to)
                    to = next(to)
                else:
                    stack.pop()
                    if not stack:
                        break
                    to = stack[-1].send(to)
            return to

    return wrapped_func


def topsort(indu: List[int], g: defaultdict) -> List:
    """
    把图g按照拓扑序列进行排序。
    :param indu: 图的入度
    :param g: 使用 defaultdict 进行建图。
    :return:
    """
    res = []
    q = deque()
    for i in range(26):
        if indu[i] == 0:
            q.append(i)
    # print(q, g)
    # print(indu)
    while q:
        u = q.popleft()
        res.append(chr(u + 97))
        for x in g[u]:
            indu[x] -= 1
            if not indu[x]:
                q.append(x)
    return res


def is_bipartite_graph(g: defaultdict, n: int) -> bool:
    """
    染色法判断一个图是不是二分图
    :param g: 图，使用 defaultdict 进行建图。
    :param n: 图中点的数量
    :return:
    """
    color = [0] * (n + 1)
    for i in range(1, n + 1):
        if color[i] != 0:
            continue
        color[i] = 1
        q = deque()
        q.append(i)
        while q:
            u = q.popleft()
            for x in g[u]:
                if not color[x]:
                    color[x] = 3 - color[u]
                    q.append(x)
                else:
                    if color[x] == color[u]:
                        return False
    return True


def input_nums():
    return list(map(int, input().split(' ')))


# n, m = input_nums()
# dx, dy = [-1, 0, 1, 0], [0, 1, 0, -1]
# cnt = 0
# nums = []
# for _ in range(n):
#     nums.append(list(input().strip()))
# vis = [[-1] * m for _ in range(n)]
# g = [0] * (n * m)
#
#
# @bootstrap
# def dfs(x, y):
#     g[cnt] += 1
#     vis[x][y] = cnt
#     for i in range(4):
#         a = x + dx[i]
#         b = y + dy[i]
#         if 0 <= a < n and 0 <= b < m and nums[a][b] == '.' and vis[a][b] == -1:
#             yield dfs(a, b)
#     yield
#
#
# for i in range(n):
#     for j in range(m):
#         if vis[i][j] == -1 and nums[i][j] == '.':
#             dfs(i, j)
#             cnt += 1
#
# for i in range(n):
#     for j in range(m):
#         if nums[i][j] == '*':
#             d = set()
#             t = 0
#             for k in range(4):
#                 a = i + dx[k]
#                 b = j + dy[k]
#                 if 0 <= a < n and 0 <= b < m and vis[a][b] not in d and nums[a][b] != '*':
#                     d.add(vis[a][b])
#                     t += g[vis[a][b]]
#             # nums[i][j] = str((t + 1) % 10)
#             print((t + 1) % 10, end="")
#         else:
#             print('.', end="")
#     print('')

# for i in range(n):
#     print(''.join(nums[i]))
#
@bootstrap
def dfs(x, y):
    global res, cnt
    vis[x][y] = cnt  # 把当前块标上号
    res += 1  # 累加器加1

    for i in range(4):
        nx = x + lx[i]
        ny = y + ly[i]  # 向四周移动
        # 判断是否不越界，是空地且没有访问过
        if (0 <= nx < n and 0 <= ny < m and
                s[nx][ny] == '.' and not vis[nx][ny]):
            yield dfs(nx, ny)  # dfs递归回溯
    yield


# 读取输入
n, m = map(int, input().split())

# 初始化数组，注意Python索引从0开始，所以需要n+1和m+1的大小
# s = [['#'] * (m + 1) for _ in range(n + 1)]
vis = [[0] * (m + 1) for _ in range(n + 1)]
ans = [0] * 5100000

# 方向数组
lx = [-1, 0, 1, 0]
ly = [0, 1, 0, -1]

# 读入地图
s = []
for _ in range(n):
    s.append(list(input().strip()))

cnt = 0  # 连通块编号
res = 0  # 当前连通块大小

# 寻找并标记所有连通块
for i in range(n):
    for j in range(m):
        if s[i][j] == '.' and not vis[i][j]:
            res = 0  # res存储当前部分连通块个数
            cnt += 1  # cnt表示这一部分连通块的编号
            dfs(i, j)  # dfs搜索，把与当前块联通的'.'全部标记
            ans[cnt] = res  # 第cnt部分连通块的个数是res

# 输出结果
for i in range(n):
    row = []
    for j in range(m):
        if s[i][j] == '*':
            f = [0] * 4  # 存储四周连通块的编号
            sum_blocks = 0  # 当前块的答案

            # 遍历四个方向
            for k in range(4):
                x = i + lx[k]
                y = j + ly[k]

                # 如果是墙或者已经计算过的连通块则跳过
                # print('xy', x, y)
                if (0 <= x < n and 0 <= y < m and
                s[x][y] == '#' ):
                    continue
                if vis[x][y] in f:
                    continue

                f[k] = vis[x][y]  # 标记编号
                sum_blocks += ans[vis[x][y]]  # 答案加上这个连通块的大小

            row.append(str((sum_blocks + 1) % 10))  # 模10输出（要记得加上本身）
        else:
            row.append('.')  # 如果是空地就直接输出'.'

    print(''.join(row))  # 输出一行
