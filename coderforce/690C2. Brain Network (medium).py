from collections import defaultdict


n, m = map(int, input().split(' '))
g = defaultdict(list)
for _ in range(m):
    a, b = map(int, input().split(' '))
    g[a].append(b)
    g[b].append(a)


def dfs(start):
    global d
    stack = [(start, 0)]  # (node, state)

    while stack:
        u, state = stack[-1]

        if state == 0:
            vis[u] = 1
            stack[-1] = (u, 1)
            for x in g[u]:
                if not vis[x]:
                    stack.append((x, 0))
        else:
            # max_child = 0
            for x in g[u]:
                if vis[x]: continue
                    # max_child = max(max_child, f[x] + 1)
                d = max(d, f[u] + f[x] + 1)
                f[u] = max(f[u], f[x] + 1)
                stack.pop()


vis = [0] * (n + 1)
f = [0] * (n + 1)
d = -1
dfs(1)
print(d)

















