n = int(input())
nums = list(map(int, input().split(' ')))
t = input()
maxx = 0
for i in range(n - 1):
    maxx = max(maxx, nums[i])
    # print(maxx, i)
    if t[i] == '0':
        if i + 1 != maxx:
            print("NO")
            exit()
print("YES")


