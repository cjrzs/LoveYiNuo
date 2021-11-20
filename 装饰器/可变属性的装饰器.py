import time
from functools import wraps, partial
import logging


def attach_wrapper(obj, func=None):
    if func is None:
        return partial(attach_wrapper, obj)
    setattr(obj, func.__name__, func)
    return func


def logger(level, name=None, message=None):

    def decorate(func):
        logname = name if name else func.__module__
        log = logging.getLogger(logname)
        logmsg = message if message else func.__name__

        @wraps(func)
        def wrapper(*args, **kwargs):
            log.log(level, logmsg)
            return func(*args, **kwargs)

        @attach_wrapper(wrapper)
        def set_level(newlevel):
            nonlocal level
            level = newlevel

        @attach_wrapper(wrapper)
        def set_message(newmsg):
            nonlocal logmsg
            logmsg = newmsg


        return wrapper
    return decorate


def timer(func):

    @wraps(func)
    def decorator(*args, **kwargs):
        start = time.time()
        res = func(*args, **kwargs)
        print(f'耗时{time.time() - start}')
        return res
    return decorator


@timer
@logger(logging.DEBUG)
def add(x, y):
    return x + y


@logger(logging.CRITICAL, 'cjr')
def spam():
    print(f'Spam!')


logging.basicConfig(level=logging.DEBUG)
# print(add(4, 5))

add.set_message('打印日志')
add.logmsg = '打印日志'
t = add(4, 6)
print(t)



