from queue import Queue
from threading import Thread

_sentinel = object()


def producer(out_q, data):
    while data:
        data -= 1
        out_q.put(data)
    out_q.put(_sentinel)


def consumer(in_q):
    while True:
        data = in_q.get()
        print(data)
        if data is _sentinel:
            in_q.put(_sentinel)
            break


# queue 是线程安全的队列，因此无需加锁，可以直接在多线程环境中使用
q = Queue()
t1 = Thread(target=producer, args=(q, 10))
t2 = Thread(target=consumer, args=(q, ))
t1.start()
t2.start()

