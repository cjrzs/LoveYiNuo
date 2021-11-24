"""
concurrent.futures库中有实现好的线程池可以使用
"""
from socket import socket, AF_INET, SOCK_STREAM
from concurrent.futures import ThreadPoolExecutor


def echo_client(sock, client_addr):
    print('got connection from', client_addr)
    while True:
        msg = sock.recv(65536)
        if not msg:
            break
    print('client closed connection')
    sock.close()


def echo_server(addr):
    pool = ThreadPoolExecutor(128)
    sock = socket(AF_INET, SOCK_STREAM)
    sock.bind(addr)
    sock.listen(5)
    while True:
        client_sock, client_addr = sock.accept()
        pool.submit(echo_client, client_sock, client_addr)


echo_server(('', 15000))

