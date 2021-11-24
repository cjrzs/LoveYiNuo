import threading


values = []
lock = threading.Lock()


def incr(delta=1):
    value = 10
    with lock:  # 在with块中拥有锁 离开with时候释放锁
        while value < 20:
            value += delta
            values.append(value)
            print(values)


def decr(delta=1):
    value = 10
    with lock:  # 在with块中拥有锁 离开with时候释放锁
        while value:
            value -= delta
            values.append(value)
            print(values)


t1 = threading.Thread(target=incr)
t2 = threading.Thread(target=decr)
t1.start()
t2.start()


