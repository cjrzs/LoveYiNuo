"""
出租车队运营事件仿真
"""
import collections
import queue

DEPARTURE_INTERVAL = 5

Event = collections.namedtuple('Event', 'time proc action')  # 事件仿真模型


def taxi_process(ident, trips, start_time=0):
    """
    出租车运行进程
    :param ident:
    :param trips:
    :param start_time:
    :return:
    """
    time = yield Event(start_time, ident, '离开车库')
    for i in range(trips):
        time = yield Event(time, ident, '乘客上车')
        time = yield Event(time, ident, '乘客下车')
    yield Event(start_time, ident, '结束运行')


def compute_duration(previous_action):
    if previous_action == '离开车库':
        return 3
    elif previous_action == '乘客上车':
        return 5
    elif previous_action == '乘客下车':
        return 1
    else:
        return 0


class Simulator:

    def __init__(self, proc_map):
        self.events = queue.PriorityQueue()
        self.procs = dict(proc_map)

    def run(self, end_time):
        # 预激协程 让所有车都离开车库 （停在第一个yield）
        for _, proc in sorted(self.procs.items()):
            first_event = next(proc)
            self.events.put(first_event)

        sim_time = 0
        while sim_time < end_time:
            # 队列中没有事件 则循环结束
            if self.events.empty():
                print('整个事件结束')
                break
            current_event = self.events.get()  # 从事件队列中取出time属性最小的对象
            sim_time, proc_id, previous_action = current_event
            print(f'taxi {proc_id} {current_event}')
            active_proc = self.procs[proc_id]  # 从字典中获取当前车的协程
            next_time = sim_time + compute_duration(previous_action)  # 计算下一个操作所需时间
            try:
                next_event = active_proc.send(next_time)  # 发送给协成 产出下一事件
            except StopIteration:
                del self.procs[proc_id]  # 抛异常说明执行完毕 则从字典中删除当前协程
            else:
                self.events.put(next_event)  # 否则把新事件放到时间循环中
        else:
            print(f'运营时间结束，还有 {self.events.qsize()} 个事件')


num_taxis = 3
end_time = 100
taxis = {i: taxi_process(i, (i + 1) * 2, i * DEPARTURE_INTERVAL) for i in range(num_taxis)}

sim = Simulator(taxis)
sim.run(end_time)


