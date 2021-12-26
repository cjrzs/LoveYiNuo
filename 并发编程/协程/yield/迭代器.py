# 1、iter(iterable) 以一个可迭代对象作为参数
iter_obj = [1, 2, 3]  # list可能是我们最熟悉的可迭代对象
list_iter = iter(iter_obj)
print(list_iter)  # <list_iterator object ...>
print(hasattr(list_iter , '__next__'))  # True

print(f'next函数迭代：{next(list_iter)}')  # next函数迭代：1
for item in list_iter:
    print(f'for循环迭代：{item}')  # for循环迭代：2  for循环迭代：3
# next(list_iter)  # StopIteration


# 2、iter(callable, sentinel) 以一个可调用对象做参数 并设置哨兵
def call_obj():  # 定义一个函数作为可调用对象
    return [1, 2, 3]
call_iter = iter(call_obj, 2)
print(call_iter)  # <callable_iterator object ...>
print(hasattr(call_iter , '__next__'))  # True

data = {
    'Jinx': '爆爆',
    'Vi': '蔚',
    'Caitlyn': '小蛋糕',
    'Jayce': '议员',
    'Viktor': '光荣进化'
}
if 'Jinx' in data:
    print(data['Jinx'])  # 爆爆
for k in data:
    print(k)  # Jinx Vi Caitlyn Jayce Viktor
for k in data.keys():
    print(k)  # Jinx Vi Caitlyn Jayce Viktor
iterator = iter(data)
# print(next(iterator)) #
# print(next(iterator))
# print(next(iterator))
# print(next(iterator))
# print(next(iterator))
# print(next(iterator)) # StopIteration
for item in iterator:
    print(item)

class MyDict(dict):
    def __init__(self, data):
        super().__init__(data)
        self.idx = 0

    def __iter__(self):
        return self

    def __next__(self):
        try:
            item = list(self.values())[self.idx]
        except IndexError:
            raise StopIteration
        self.idx += 1
        return item


iter2 = iter(MyDict(data))
print(next(iter2))  # 爆爆
print(next(iter2))  # 蔚
for item in iter2:
    print(item)  # 小蛋糕 议员 光荣进化
print(next(iter2))
print(hasattr(list, '__iter__'))
print(hasattr(list, '__next__'))

l = [1, 2, 3]
for i in l:
    print(i)
# next(l)

