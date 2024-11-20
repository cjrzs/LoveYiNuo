from math import sqrt, pow

n, x1, y1, x2, y2 = list(map(int, input().split(' ')))


r = abs(x1 - x2)
c = abs(y1 - y2)

# 如果横纵坐标之一的距离差 等于边长，说明：
# 两个点是隔着 整条边的。
if r == n or c == n:
    res = x1 + x2 + y1 + y2
else:
    res = r + c
print(min(res, 4 * n - res))
