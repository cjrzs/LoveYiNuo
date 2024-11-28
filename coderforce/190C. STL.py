from collections import deque, Counter

n = int(input())
nums = input().split(' ')
q = deque()
q.extend(nums)
res = []
stack = []
i = 0
for i, cur in enumerate(nums):
    res.append(cur)
    if cur == 'pair':
        res.append('<')
        stack.append(2)
    else:
        while stack:
            if stack[-1] == 2:
                res.append(',')
                stack[-1] = 1
                break
            if stack[-1] == 1:
                res.append('>')
                stack.pop()

    if (not stack and i < len(nums) - 1) or (stack and i == len(nums) - 1):
        print("Error occurred")
        exit()

print(''.join(res))










