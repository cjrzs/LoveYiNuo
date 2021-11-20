from functools import partial


def func1(a, b, c, d):
    print({'a': a, 'b': b, 'c': c, 'd': d})


func1 = partial(func1, 1)
print(func1)
func1(2, 3, 4)

func1 = partial(func1, d=5)
print(func1)
func1(2, 3)
func1(2, 3, 4)


