import sys
import threading
from collections import defaultdict
from threading import Thread
from typing import Dict, List, Set


def main() -> None:
    n: int = int(input())
    k: int = int(input())
    favorites: Dict[int, Set[int]] = defaultdict(set)
    for _ in range(k):
        a, b = map(int, input().split(' '))
        favorites[a].add(b)
        favorites[b].add(a)

    hates: Dict[int, Set[int]] = defaultdict(set)
    m: int = int(input())
    for _ in range(m):
        a, b = map(int, input().split(' '))
        hates[a].add(b)
        hates[b].add(a)

    vis: List[bool] = [False] * (n + 1)

    def dfs(u: int) -> None:
        nonlocal hate_group
        like_group.add(u)
        for x in favorites[u]:
            if not vis[x]:
                vis[x] = True
                hate_group |= hates[x]
                dfs(x)

    res: int = 0
    for i in range(1, n + 1):
        if not vis[i]:
            hate_group: Set[int] = hates[i]
            like_group: Set[int] = set()
            vis[i] = True
            dfs(i)
            if not hate_group & like_group:
                res = max(res, len(like_group))

    print(res)


sys.setrecursionlimit(10 ** 6)
threading.stack_size(1 << 27)
thread: Thread = threading.Thread(target=main)
thread.start()
thread.join()







