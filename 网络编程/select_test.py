# IO多路复用 select
from typing import Dict

import select
import socket
import queue

sk = socket.socket()
sk.setblocking(False)  # 设置非阻塞
address = ('localhost', 12345)
sk.bind(address)
sk.listen()

inputs = [sk]  # 用于检测读入的套接字
outputs = []  # 用于检测写出事件的套接字
message: Dict[socket: queue.Queue] = {}  # 存储信息


def socket_close(s: socket):
    """关闭一个套接字"""
    if s in outputs:
        outputs.remove(s)
    inputs.remove(s)
    s.close()
    del message[s]


def read_event(s: socket):
    """读事件处理流程"""
    # 如果是新的套接字是 主连接的套接字
    if s is sk:
        connection, client_info = s.accept()  # 该套接字监控其他连接
        connection.setblocking(False)  # 改成非阻塞
        inputs.append(connection)  # 添加该连接到监控
        message[connection] = queue.Queue()  # 给它准备好要发送的数据队列
    else:
        # 与已发送数据的客户端建立连接 获取已发送数据
        data = s.recv(1024)
        if data:
            message[s].put(data)  # 处理接收到的信息
            if s not in outputs:
                outputs.append(s)
        else:
            # 没有数据关闭连接
            socket_close(s)


def write_event(s: socket):
    """写事件处理流程"""
    try:
        # 非阻塞获取该套接字上的任务
        next_msg = message[s].get_nowait()
    except queue.Empty:
        outputs.remove(s)  # 如果该套接字上任务全完成 移出该套接字
    else:
        s.send(next_msg)  # 发送消息


while inputs:
    readable, writable, exceptional = select.select(inputs, outputs, inputs)
    for s in readable:
        read_event(s)
    for s in writable:
        writable(s)
    for s in exceptional:
        socket_close(s)




