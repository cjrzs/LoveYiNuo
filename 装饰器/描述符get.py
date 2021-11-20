class Foo:
    def __init__(self, x):
        self.x = x

    def __get__(self, instance, owner):
        print(self.x)


class A:
    name = Foo(23)


A.name  # 23


