def find(x):
    if x != p[x]:
        p[x] = find(p[x])
    return p[x]


def merge(a, b):
    x, y = find(a), find(b)
    if x != y:
        p[x] = y
        d[y] = (min(d[x][0], d[y][0]), max(d[x][1], d[y][1]))
        d[x] = (float('inf'), float('-inf'))


n = int(input())
d = [(float('inf'), float('-inf'))] * (n + 1)
p = list(range(n + 1))
# nums = []
i = 1
for _ in range(n):
    t, a, b = map(int, input().split(' '))
    # nums.append((t, a, b))
    if t == 1:
        node = find(i)
        d[node] = (a, b)
        for j in range(1, i):
            start = find(j)
            if a < d[start][0] < b or a < d[start][1] < b:
                merge(node, start)
        i += 1
    elif t == 2:
        if find(a) != find(b):
            print("NO")
        else:
            print("YES")
#
# for i in range(1, n + 1):
#     print(find(i), end=" ")







