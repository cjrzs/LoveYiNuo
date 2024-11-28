# def dfs(u):
#     val = 1 if s[u - 1] == 'W' else -1
#     if u not in g:
#         return val
#     for x in g[u]:
#         val += dfs(x)
#     if val == 0:
#         global res
#         res += 1
#     return val

T = int(input())
for _ in range(T):
    n = int(input())
    nums = list(map(int, input().split(' ')))
    d = [0] * n
    s = input()
    res = 0
    for i in range(n - 1, -1, -1):
        d[i] += 1 if s[i] == 'W' else -1
        print(f'nums[i - 1]: {nums[i - 1]}')
        if i > 0:
            d[nums[i - 1] - 1] += d[i]
    print(d.count(0))



