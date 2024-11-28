n = int(input())
nums = set(map(int, input().split(' ')))

# print(nums)


def dfs(u, level):
    # print(u, level)
    global res
    if u in vis:
        return
    vis.add((u, level))
    if level == n:
        res = list(vis)
        # print('res', res)
        return
    if res:
        return
    t1 = u * 2
    t2 = u // 3
    p = u % 3
    if t1 in nums:
        dfs(t1, level + 1)
    if t2 in nums and p == 0:
        dfs(t2, level + 1)


res = None
for x in nums:
    if res:
        break
    # print('------------')
    vis = set()
    dfs(x, 1)

res.sort(key=lambda x: x[1])
print(' '.join([str(x[0]) for x in res]))



