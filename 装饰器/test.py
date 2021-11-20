import time
import typing as t
from functools import lru_cache, partial


def timer(func):
    def decorator(*args, **kwargs):
        start = time.time()
        res = func(*args, **kwargs)
        print(f'函数 {func.__name__} 总耗时： {time.time() - start}')
        return res
    return decorator


def adder(a):
    def decorator():
        a = 3
        print(a)  # 3
    print(a)  # 5
    return decorator


# a = adder(5)


# flask 源码
def route(self, rule: str, **options: t.Any) -> t.Callable:
    def decorator(f: t.Callable) -> t.Callable:
        endpoint = options.pop("endpoint", None)
        self.add_url_rule(rule, endpoint, f, **options)
        return f

    return decorator


def hello_world():
    return 'Hello World!'
# 调用route函数，执行后获得函数 decorator
# decorator = route('/')
# print(decorator)  # <function route.<locals>.decorator at 0x000001EF72B3B040>
# # 把函数hello_world作为参数 传入到函数decorator 并执行
# # 得到新的视图函数hello_world
# hello_world = decorator(hello_world)


def test(func=None, *, b=2):
    # print('func', func)
    if func is None:
        return partial(test, b=b)
    # print(b)
    def deco():
        pass
        # print(f'这是一个装饰器')
    return deco


@test(b=3)
def target():
    pass
    # print(f'这是被装饰函数')
# target = test(target)

# target() # 这是一个装饰器
# print(target())  # <function test.<locals>.decorator ...>


start = time.time()
@lru_cache()
def fibonacci(n):
    if n < 3:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


# print(fibonacci(30))
# print(time.time() - start)  # 0.0009968280792236328


def test(func=None, b=3):
    if func is None:
        return partial(test, b=b)
    print(func)
    print(b)
    def deco(*args):
        print(args)
        print(f'这是一个装饰器')
    return deco


# @test(b=3)
def target1(k):
    print(f'这是被装饰函数{k}')
a = test(b=3)
print(a)
a = a(target1)(2)
print(a)
