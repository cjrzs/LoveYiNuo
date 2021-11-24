"""
使用Event控制线程的同步执行
1、Event.set() 设置标志为True。使所有处于等待阻塞状态的线程恢复运行
2、Event.is_set()  查看标识是否为True。是的话返回True。
3、Event.clear()  设置标志为False.
4、Event.wait()  标志位是True的时候立即返回。阻塞线程 等待其他线程调用set。
"""


from threading import Thread, Event
import time


def countdown(n, started_evt):
    print('countdown starting')
    started_evt.set()
    while n > 0:
        print(f'T-minus {n}')
        n -= 1
        time.sleep(0.1)


started_evt = Event()
print(f'launching countdown')
t = Thread(target=countdown, args=(10, started_evt))
t.start()
started_evt.wait()
print(f'countdown is running')

