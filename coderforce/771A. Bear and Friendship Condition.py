# def find(x):
#     if p[x] != x:
#         p[x] = find(p[x])
#     return p[x]
#
#
# def merge(a, b):
#     x = find(a)
#     y = find(b)
#     if x != y:
#         p[x] = y
#         nums[y] += nums[x]
#         nums[x] = 0
#
#
n, m = map(int, input().split(' '))
# p = [0] * (n + 1)
# nums = [0] * (n + 1)
# for i in range(1, n + 1):
#     p[i] = i
#     nums[i] = 1
# # print(n, m)
#

g = [set([i]) for i in range(n + 1)]
vis = [0] * (n + 1)
for _ in range(m):
    a, b = map(int, input().split(' '))
    g[a].add(b)
    g[b].add(a)
    # merge(a, b)


for i in range(1, n + 1):
    if not vis[i]:
        for j in g[i]:
            vis[j] = 1
            if g[i] != g[j]:
                print("NO")
                exit()
print("YES")









