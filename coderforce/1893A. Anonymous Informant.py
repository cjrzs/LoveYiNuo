T = int(input())


def solve(n, k, nums):
    cur = n
    vis = set()
    while k and cur not in vis:
        vis.add(cur)
        if nums[cur] > n:
            return "NO"
        # print(n, cur, nums[cur])
        cur = (cur - nums[cur] + n) % n
        # print('---', cur)
        k -= 1
    return "YES"


for _ in range(T):
    n, k = list(map(int, input().split(' ')))
    nums = [0] + list(map(int, input().split(' ')))
    print(solve(n, k, nums))




