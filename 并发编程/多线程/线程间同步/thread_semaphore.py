"""
使用信号量  单独唤醒线程 或者设置有个数限制的并发Semaphore(5) 设置最大并发数为5
"""
import threading


sema = threading.Semaphore()


def worker(n, sema: threading.Semaphore):
    sema.acquire()
    print(f'working {n}')


nworkers = 10

for i in range(nworkers):
    t = threading.Thread(target=worker, args=(i, sema))
    t.start()

print(sema.release())  # 释放锁 只能释放一个锁
print(sema.release())
print(sema.release())

