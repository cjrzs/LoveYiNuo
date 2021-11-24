"""
在该示例中，锁作用于整个类，而不是于每个类的示例
现在这个锁可以用来同步类中的方法。

并且在类中已经持有锁的方法，可以调用同样持有该锁的方法，例如decr()。

该类无论有多少个实例，它始终都是只有一个锁。因此当有大量的实例时候，使用类属性的锁让内存使用效率高的多。

缺点是当大量的线程频繁更新实例对象的时候，会产生更多的锁竞争。
"""

import threading


class ShareCounter:

    _lock = threading.Lock()

    def __init__(self, init_val):
        self._value = init_val

    def incr(self, delta=1):
        with ShareCounter._lock:
            self._value += delta

    def decr(self, delta=1):
        with ShareCounter._lock:
            self.incr(-delta)


