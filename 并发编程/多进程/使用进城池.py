"""
分析日志文件，找出所有访问过 robots.txt 的主机
"""


import gzip
import io
import glob
from concurrent import futures


def find_robots(filename):
    robots = set()
    with gzip.open(filename) as f:
        # 从IO流中读数据 并解析 把访问了robots的主机放到robots中
        for line in io.TextIOWrapper(f, encoding='ascii'):
            fields = line.split()
            if fields[6] == 'robots.txt':
                robots.add(fields[0])
    return robots


def find_all_robots(logdir):
    # glob 会确定匹配 路径的列表
    files = glob.glob(logdir + '/*.log.gz')
    all_robots = set()
    # 使用进程池
    with futures.ProcessPoolExecutor() as pool:
        for robots in pool.map(find_robots, files):
            all_robots.update(robots)
    return all_robots


