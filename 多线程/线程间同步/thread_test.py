import socket
import time
from threading import Thread

def countdown(n):
    while n > 0:
        print(f'T-minus {n}')
        n -= 1
        time.sleep(0.5)


# 使用一个线程执行countdown
# t1 = Thread(target=countdown, args=(10, ))
# t1.start()  # 启动后会直接执行函数 相当于countdown()
# t1.join()  # 连接到线程t  即只有该线程结束才会执行后面的代码
# 判断线程状态
# if t1.is_alive():
#     print(f'still running')
# else:
#     print(f'completed')

# daemon 守护线程 后台运行 无法join
# t = Thread(target=countdown, args=(10, ), daemon=True)
# t.start()


class CountdownTask:
    """让线程在指定点上退出"""
    def __init__(self):
        self.__running = True

    def terminate(self):
        self.__running = False

    def run(self, n, sock):
        sock.settimeout(5)
        while self.__running and n > 0:
            try:
                sock.rece(1024)
            # 在涉及IO操作的线程中 必须考虑IO阻塞的问题 因此必须加上超时循环
            except socket.timeout:
                continue
            print(f'T-minus {n}')
            n -= 1
            time.sleep(0.1)


c = CountdownTask()
t = Thread(target=c.run, args=(10,))
t.start()
c.terminate()











