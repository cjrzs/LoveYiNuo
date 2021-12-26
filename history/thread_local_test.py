from threading import Thread, local

# 全局初始化一个local实例
mydata = local()
# 主线程中，给该实例设置一个属性number为42
mydata.number = 42
print(mydata.number)  # 42

# 还可以用访问__dict__的方式访问local对象的属性字典
print(mydata.__dict__)  # {'number': 42}
# 再给主线程的mydata设置一个带有默认值的属性
mydata.__dict__.setdefault('widgets', [])
print(mydata.widgets)  # []


# 主线程中创建全局变量log及函数func
log = []
# 函数func的作用很简单，将mydata的数据排序后放到log中
def func():
    items = sorted(mydata.__dict__.items())
    log.append(items)  # 首次放入数据
    mydata.number = 11
    log.append(mydata.number)  # 再次放入数据

# 另外启动一个线程执行func
t = Thread(target=func)
t.start()
t.join()
print(log)  # [[], 11]
print(mydata.number)  # 42

