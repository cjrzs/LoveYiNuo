import sys
import threading
from collections import defaultdict, deque


def solution():
    n, k = map(int, input().split(' '))
    major = deque(list(map(int, input().split(' '))))
    g = defaultdict(list)
    for i in range(1, n + 1):
        g[i].extend(list(map(int, input().split(' ')))[1:])
    # print(g)

    vis = [0] * (n + 1)
    res = []

    #
    def dfs(u):
        if u in q:
            print(-1)
            exit()
        q.add(u)
        # print(u, vis, q)
        for x in g[u]:
            if not vis[x]:
                dfs(x)
        vis[u] = 1
        res.append(u)

    q = set()
    while major:
        cur = major.popleft()
        if not vis[cur]:
            dfs(cur)

    print(len(res))
    print(*res)
#
#
def main():
    t = 1
    for _ in range(t):
        solution()

# main()
sys.setrecursionlimit(10**6)
threading.stack_size(1 << 27)
thread = threading.Thread(target=main)
thread.start(); thread.join()

# sys.setrecursionlimit(10**6)
# n, k = map(int, input().split(' '))
# major = deque(list(map(int, input().split(' '))))
# g = defaultdict(list)
# for i in range(1, n + 1):
#     g[i].extend(list(map(int, input().split(' ')))[1:])
# # print(g)
#
# vis = [0] * (n + 1)
# res = []
#
# #
# def dfs(u):
#     if u in q:
#         print(-1)
#         exit()
#     q.add(u)
#     # print(u, vis, q)
#     for x in g[u]:
#         if not vis[x]:
#             dfs(x)
#     vis[u] = 1
#     res.append(u)
#
# q = set()
# while major:
#     cur = major.popleft()
#     if not vis[cur]:
#         dfs(cur)
#
# print(len(res))
# print(*res)
#
#
