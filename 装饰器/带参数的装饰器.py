"""带有参数rule的装饰器"""


from functools import partial


def route(rule):
    def decorator(func, *args, **kwargs):
        print(f'这是装饰器 1 {rule}')
        return func(rule)
    return decorator


def route2(func=None, rule='/'):
    if func is None:
        return partial(route2, rule=rule)

    def decorator(*args, **kwargs):
        print(f'这是装饰器 2 {rule}')
        return func(args)
    return decorator


def route3(rule):
    def decorator(func):
        def route(*args, **kwargs):
            print(f'这是装饰器 3 {rule}')
            return func(args)
        return route
    return decorator


if __name__ == '__main__':
    @route('/')
    def target(val):
        print(f'val = {val}')

    # tmp = route2(rule='/2')
    # print(tmp)
    @route2(rule='/2')
    def target2(val):
        print(f'val = {val}')
    target2(2)


    # @route3(rule='/3')
    def target3(val):
        print(f'val = {val}')
    tmp = route3('/3')
    print(tmp)
    tmp(target3)(3)
    # target3(3)

