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


def knapsack_group_dp(n: int, m: int, v: List[List[int]], w: List[List[int]], s: List[int]):
    """
    分组DP
    物品分组，每组只能选一个，返回执行容积下，能选到的最大价值。
    :params: v[i][j] 第i组的第j个物品占用的体积
    :params: w[i][j] 第i组的第j个物品占用的价值
    :params: s[i] 第i组有几个物品
    :params: n 总共有n个分组
    :params: m 指定的总容积。
    """
    f = [0] * (m + 1)
    for i in range(n):
        for j in range(m, 0, -1):
            for k in range(s[i]):
                if v[i][k] <= j:
                    f[j] = max(f[j], f[j - v[i][k]] + w[i][k])
    return f[m]


def knapsack_01_dp(n, m, v, w):
    pass


def knapsack_multiple_dp():
    pass


def knapsack_unbounded_dp():
    pass


def knapsack_mixed_dp():
    pass


def knapsack_two_dimensional_dp():
    pass


def knapsack_dependent_dp():
    pass


def knapsack_count_solution_dp():
    pass


def knapsack_optimal_dp():
    pass


def input_nums():
    return list(map(int, input().split(' ')))


n = int(input())
target_nums = [0] + input_nums()
d = [0] + input_nums()
p = list(range(n + 1))


@bootstrap
def find(x):
    if p[x] != x:
        p[x] = yield find(p[x])
    yield p[x]


def merge(a, b):
    x = find(a)
    y = find(b)
    if x != y:
        p[x] = y


for i in range(1, n + 1):
    if i + d[i] <= n:
        merge(i, i + d[i])
    if i - d[i] > 0:
        merge(i, i - d[i])
# print(p)

for i in range(1, n + 1):
    if find(i) != find(target_nums[i]):
        print("NO")
        exit()
print("YES")




