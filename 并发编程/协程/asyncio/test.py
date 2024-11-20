import asyncio
import socket
import time
from asyncio import events


def receive():
    s = socket.socket()  # 创建一个新的套接字
    host = socket.gethostname()  # 获取本地主机名
    port = 12345  # 设置端口号
    s.connect((host, port))  # 连接
    return s.recv(1024)


async def io_opera(num):
    loop = asyncio.get_event_loop()
    msg = await loop.run_in_executor(None, receive)
    print(f'Task{num} result: {msg}')


async def sum(name, number):
    result = await io_opera(1)
    print(f'Task ({name}) result: {result}-{number}')


def main():
    tasks = [
        asyncio.ensure_future(io_opera(1)),
        asyncio.ensure_future(io_opera(2)),
        asyncio.ensure_future(io_opera(3)),
        asyncio.ensure_future(io_opera(4)),
    ]
    return tasks


if __name__ == '__main__':
    start = time.time()
    # loop = asyncio.get_event_loop()
    # tasks = [mytask('task1'), mytask('task2')]
    # loop.run_until_complete(asyncio.wait(tasks))
    # loop.close()
    tasks = main()
    loop = asyncio.get_event_loop()
    loop.run_until_complete(asyncio.wait(tasks))
    # asyncio.run(main(), debug=True)
    # asyncio.run(main(), debug=True)
    print(time.time() - start)



