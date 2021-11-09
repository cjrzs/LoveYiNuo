"""
哲学家就餐问题。

有两个线程A和B。当A持有一个锁lockA，并且等待lockB才能继续运行的时候。
B同时持有着锁lockB，同时B也在等待着lockA才能继续运行。
此时程序产生死锁。

解决死锁的一个简单办法是在多个锁的场景中，对每个锁都进行编号，然后需要锁的线程按照编号顺序获取。
"""
import threading
import queue
import time
from contextlib import contextmanager


# local类用来管理线程，每个线程都可以对其设置属性，并且不同线程设置的属性是互不干扰的
_local = threading.local()


@contextmanager
def acquire(*locks):
    locks = sorted(locks, key=lambda x: id(x))
    acquired = getattr(_local, 'acquired', [])
    if acquired and max(id(lock) for lock in acquired) >= id(locks[0]):
        raise RuntimeError('Lock Order Violation')
    acquired.extend(locks)
    _local.acquired = acquired
    try:
        for lock in locks:
            lock.acquire()
        yield
    finally:
        for lock in reversed(locks):
            lock.release()
        del acquired[-len(locks):]


lock1 = threading.Lock()
lock2 = threading.Lock()


q = queue.Queue()


def thread_1():
    n = 0
    while True:
        with acquire(lock1, lock2):
            q.put(n)
            n += 1


def thread_2():
    while True:
        with acquire(lock2, lock1):
            data = q.get()
            print(f'{data}')


t1 = threading.Thread(target=thread_1)
t2 = threading.Thread(target=thread_2)
# 设置守护线程 后台执行 注释掉则无限循环
# t1.daemon = True
# t2.daemon = True
t1.start()
t2.start()


"""
下面例子中使用了线程本地存储来解决多个acquire()嵌套的问题。
分别acquire两个锁会出现死锁 这是因为每个线程都会记住他们已经获取到的锁的顺序。
而我们的acquire()会检查之前获取的锁的列表，先获取的锁ID必须比后获取的锁ID小。

PS：即使用下面例子中的做法 会使acquire()函数中的排序失效。
"""


def thread_3():
    n = 0
    while True:
        with acquire(lock2):
            with acquire(lock1):
                q.put(n)
                n += 1


def thread_4():
    while True:
        with acquire(lock2):
            with acquire(lock1):
                data = q.get()
                print(f'{data}')

