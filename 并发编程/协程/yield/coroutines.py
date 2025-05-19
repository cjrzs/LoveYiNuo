from dis import dis
import inspect



def simple_coro2(a):
    print(f'start a = {a}')
    b = yield a
    print(f'receive b = {b}')
    c = yield a + b
    print(f'receive c = {c}')

print(dis(simple_coro2))
lines = inspect.getsource(simple_coro2)
print(lines)
coro2 = simple_coro2(66)
print(coro2)  # <generator object simple_coro2...>
val = next(coro2)  # start a = 66
print(val)  # 66
# next(coro2)  # TypeError: unsupported operand type(s) for +: 'int' and 'NoneType'
print(coro2.send(15))
# print(coro2.send(99))


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




