def find(x):
    if p[x] != x:
        p[x] = find(p[x])
    return p[x]


def merge(u, x):
    a = find(u)
    b = find(x)
    if a != b:
        p[a] = b


MOD = 10 ** 9 + 7
T = int(input())
for _ in range(T):
    n = int(input())
    a = list(map(int, input().split(' ')))
    b = list(map(int, input().split(' ')))
    c = list(map(int, input().split(' ')))
    p = [0] * (n + 1)
    for i in range(1, n + 1):
        p[i] = i
    res = 1
    for i in range(n):

        if a[i] != b[i] and c[i] == 0:
            t1, t2 = find(a[i]), find(b[i])
            if t1 != t2:
                merge(t1, t2)
            else:
                res = res * 2 % MOD

    print(res)



#
# MOD = 10 ** 9 + 7
# T = int(input())
# for _ in range(T):
#     n = int(input())
#     a = list(map(int, input().split(' ')))
#     a1 = {x: i for i, x in enumerate(a)}
#     b = list(map(int, input().split(' ')))
#     b1 = {x: i for i, x in enumerate(b)}
#     c = list(map(int, input().split(' ')))
#     p = [0] * (n * 2 + 1)
#     for i in range(1, n * 2 + 1):
#         p[i] = i
#     vis = set()
#     nums = [a, b]
#     nums2 = [a1, b1]
#     for i in range(n):
#         idx = 0
#         t = i
#         val = nums[idx][t]
#         # print('初始的下标', t)
#         if val not in vis:
#             vis.add(val)
#             idx = 1 - idx  # 1
#             val = nums[idx][t]
#             # print("--", t, val)
#             while val not in vis:
#                 # print('被merge的下标', t)
#                 vis.add(val)
#                 idx = 1 - idx # 0
#                 t = nums2[idx][val]
#                 idx = 1 - idx # 1
#                 val = nums[idx][t]
#                 # print(t, val, vis)
#                 merge(t + 1, t + n + 1)
#
#     # print(p)
#     res = 1
#     for i in range(1, n + 1):
#         if p[i] == i and c[i - 1] == 0 and a[i - 1] != b[i - 1]:
#             res *= 2
#             res %= MOD
#     print(res)
#     # print('-------')

# 1
# 6
# 1 5 2 4 6 3
# 6 5 3 1 4 2
# 6 0 0 0 0 0








