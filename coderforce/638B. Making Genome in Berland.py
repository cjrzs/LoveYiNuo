from typing import Set, List

n = int(input())
pres = [-1] * 26
nexts = [-1] * 26
chars: Set[int] = set()
for _ in range(n):
    s = input()
    m = len(s)
    for i in range(m):
        cur = ord(s[i]) - 97
        chars.add(cur)
        if i > 0:
            pre = ord(s[i - 1]) - 97
            pres[cur] = pre
        if i < m - 1:
            nxt = ord(s[i + 1]) - 97
            nexts[cur] = nxt


res = ""
for i in chars:
    if pres[i] == -1:
        t: List[str] = [chr(i + 97)]
        u = i
        while nexts[u] != -1:
            u = nexts[u]
            t.append(chr(u + 97))
        res += "".join(t)
print(res)







