"""
使用Event 当消费者处理了特定数据之后通知生产者
在该例子中 生产者生产10个数据 当消费者 消费到2的时候通知生产者
"""
from queue import Queue
from threading import Thread, Event


def producer(out_q, data):
    env = Event()
    while data:
        data -= 1
        out_q.put((data, env))
    env.wait()
    print(f'执行到了2')


def consumer(in_q):
    while True:
        data, env = in_q.get()
        print(data)
        if data == 2:
            env.set()
            break


q = Queue()
t1 = Thread(target=producer, args=(q, 10))
t2 = Thread(target=consumer, args=(q, ))
t1.start()
t2.start()

