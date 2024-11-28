n, k = map(int, input().split(' '))
nums = [-1] + list(map(int, input().split(' ')))

vis = [False] * (n + 1)
used = []
used_idx = []
flag_num = 0
flag_idx = 0


def dfs(u):
    if u != k:
        pass

for i in range(1, n + 1):
    if not vis[i]:
        dfs(i)

print(used, used_idx)
