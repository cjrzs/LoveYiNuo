import socket


s = socket.socket()  # 创建一个新的套接字
host = socket.gethostname()  # 获取本地主机名
port = 12345  # 设置端口号

s.connect((host, port))  # 连接
print(s.recv(1024))  # 接收消息
s.close()


