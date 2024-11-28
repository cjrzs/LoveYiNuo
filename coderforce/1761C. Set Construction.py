T = int(input())
for _ in range(T):
    n = int(input())
    nums = []
    for _ in range(n):
        nums.append(list(input()))
    res = [[0] * (n + 1) for _ in range(n + 1)]
    for i in range(1, n + 1):
        res[i][i] = i
    for i in range(n):
        for j in range(n):
            if nums[i][j] == '1':
                res[j + 1][i + 1] = res[i + 1][i + 1]
    for i in range(1, n + 1):
        t = [str(x) for x in res[i] if x != 0]
        print(len(t), " ".join(t))







