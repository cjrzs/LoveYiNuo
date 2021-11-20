import dis


def timer(func):

    def decorator(*args, **kwargs):
        # do something
        res = func(*args, **kwargs)
        # do something
        return res
    return decorator


@timer
def adder(a, b):
    return a + b


print(dis.dis(adder))

