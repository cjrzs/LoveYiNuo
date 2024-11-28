from collections import Counter

n = int(input())
nums = list(map(int, input().split(' ')))


g = {1: 0}
for i, x in enumerate(nums):
    g[i + 2] = g[x] + 1


d = Counter(g.values())
# print(d)

res = 0
for x in d.values():
    if x % 2 != 0:
        res += 1
print(res)




