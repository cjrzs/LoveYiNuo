import socket


s = socket.socket()  # 创建一个新的套接字
host = socket.gethostname()  # 获取本地主机名
port = 12345                # 设置端口
s.bind((host, port))        # 绑定端口

s.listen(5)  # 监听套接字
while True:
    c, addr = s.accept()  # 接受客户端连接
    print(f'连接地址：{addr}')
    c.send(b'hello world')  # 有连接的客户端 给他发送消息
    c.close()

