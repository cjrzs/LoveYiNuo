T = int(input())


def check(x, c):
    vis = set()
    vis.add(x)
    while True:
        vis.add(c)
        # print(c, vis, d)
        if c not in d:
            return True
        c = d[c]
        if c in vis:
            if len(vis) < 26:
                return False
            else:
                return True


for _ in range(T):
    n = int(input())
    a = input()
    d = {}
    used = set()
    res = ''
    for x in a:
        # print("当前循环的：", x)
        if x not in d:
            for q in range(97, 123):
                c = chr(q)
                flag = check(x, c)
                # print(x, c, c != x, flag)
                if c != x and c not in used and check(x, c):
                    d[x] = c
                    used.add(c)
                    break
            res += d[x]
        else:
            res += d[x]

    print(res)
    # print("----")

