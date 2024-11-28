import socket
import time

s = socket.socket()  # 创建一个新的套接字
host = socket.gethostname()  # 获取本地主机名
port = 12345                # 设置端口
s.bind((host, port))        # 绑定端口

s.listen(5)  # 监听套接字
while True:
    c, addr = s.accept()  # 接受客户端连接
    print(f'连接地址：{addr}')
    time.sleep(1)
    c.send(b'exec?')
    c.close()

