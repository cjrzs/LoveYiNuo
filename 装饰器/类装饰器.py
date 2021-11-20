import time
import types
from functools import wraps


def timer(func):

    @wraps(func)
    def decorator(*args, **kwargs):
        start = time.time()
        res = func(*args, **kwargs)
        print(f'耗时{time.time() - start}')
        return res
    return decorator


class Timer:
    def __init__(self, func):
        wraps(func)(self)
        self.consume_time = {}

    def __call__(self, *args, **kwargs):
        print(f'self: {self}')
        print(f'self.__wrapped__: {self.__wrapped__}')
        print(f'args: {args}')
        start = time.time()
        res = self.__wrapped__(*args, **kwargs)
        self.consume_time[args] = time.time() - start
        return res

    def __get__(self, instance, owner):
        print({
            'instance': instance,
            'owner': owner,
            'self': self
        })
        if instance is None:
            return self
        return types.MethodType(self, instance)


class A:

    @Timer
    def add(self, x, y):
        time.sleep(0.1)
        return x + y
# Timer(Spam2.add(3, 5))


class Spam:

    @Timer
    def spam(self, x):
        print(self, x)
# Spam.spam(4)

# @Timer
def add(x, y):
    time.sleep(0.1)
    return x + y


# s = Spam()
# s.spam(2)

a = A()
# res = s2.add(4, 5)
res2 = a.add(5, 6)
# print(res2)
# A.add(6, 7)
print('1', A.add)
print(a.add)
print('2', a.add.consume_time)
print('3', A.add.consume_time)


# add(4, 5)
