from collections import namedtuple


Result = namedtuple('Result', 'count average')


def averager():
    total = 0.0
    cnt = 0
    average = None
    while True:
        term = yield average
        if not term:
            break
        total += term
        cnt += 1
        average = total / cnt
    return Result(cnt, average)

coro_avg = averager()
next(coro_avg)
print(coro_avg.send(10))
print(coro_avg.send(20))
try:
    print(coro_avg.send(None))
except StopIteration as e:
    print(e.value)  # PEP380 生成器函数的返回值 Result(count=2, average=15.0)

