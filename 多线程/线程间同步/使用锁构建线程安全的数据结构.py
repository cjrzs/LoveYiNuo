"""使用条件变量 将heapq变成线程安全的优先队列"""


import heapq
import  threading


class PriorityQueue:
    def __init__(self):
        self._queue = []
        self._count = 0
        self._cv = threading.Condition()

    def put(self, item, priority):
        with self._cv:  # 获取锁
            heapq.heappush(self._queue, (-priority, self._count, item))  # 将数据入堆
            self._count += 1
            self._cv.notify()  # 放完数据后 唤醒处于等待状态的线程

    def get(self):
        with self._cv:
            while len(self._queue) == 0:  # 没有数据时候阻塞该线程
                self._cv.wait()
            return heapq.heappop(self._queue)[-1]




