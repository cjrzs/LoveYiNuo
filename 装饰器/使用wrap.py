import time
from functools import wraps


def timer(func):

    @wraps(func)
    def decorator(*args, **kwargs):
        start = time.time()
        res = func(*args, **kwargs)
        print(f'耗时{time.time() - start}')
        return res
    return decorator


# @timer
def adder(a, b):
    """
    这是一个加法
    """
    return a + b


def sub(a, b):
    return a - b


print(adder.__doc__)
res = adder(3, 4)



