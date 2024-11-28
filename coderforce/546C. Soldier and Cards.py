from collections import deque

n = int(input())
nums1 = list(map(int, input().split(' ')))[1: ]
nums2 = list(map(int, input().split(' ')))[1: ]


flag = False
vis = set()
q1 = deque(nums1)
q2 = deque(nums2)
res = 0


def dfs(x, v):
    # print(x, v, q1, q2)

    global res
    res += 1
    if x > v:
        q1.append(v)
        q1.append(x)
    else:
        q2.append(x)
        q2.append(v)
    if not q1 or not q2:
        return
    if (x, v, tuple(q1), tuple(q2)) in vis:
        global flag
        flag = True
        return
    vis.add((x, v, tuple(q1), tuple(q2)))
    dfs(q1.popleft(), q2.popleft())


x, v =  q1.popleft(), q2.popleft()
dfs(x, v)
if flag:
    print(-1)
else:
    print(res, 1 if not q2 else 2)



