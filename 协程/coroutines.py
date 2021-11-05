def simple_coro2(a):
    print(f'start a = {a}')
    b = yield a
    print(f'receive b = {b}')
    c = yield a + b
    print(f'receive c = {c}')


# coro2 = simple_coro2(14)
# print(coro2)
# next(coro2)
# coro2.send(15)
# coro2.send(99)


def averager():
    total = 0.0
    cnt = 0
    average = None
    while True:
        term = yield average
        total += term
        cnt += 1
        average = total / cnt


coro_avg = averager()
next(coro_avg)
print(coro_avg.send(10))
print(coro_avg.send(20))


from functools import wraps


def coroutines(func):
    """
    预激协程的装饰器
    :param func:
    :return:
    """
    @wraps(func)
    def prime(*args, **kwargs):
        gen = func(*args, **kwargs)
        next(gen)
        return gen
    return prime




