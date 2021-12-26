import weakref
import sys


s1 = {1, 2, 3}  # 创建个对象 {1， 2， 3} 起一个名字叫s1
s2 = s1  # 给对象{1， 2， 3} 增加一个名字叫 s2
print(sys.getrefcount(s1))  # 3
# 设定一个函数bye用来在对象删除的时候触发
def bye():
    print(f'拜拜了，您内~')

ender = weakref.finalize(s1, bye)  # 设置一个回调函数 此处引用s1是弱引用（不会影响对象回收）
print(ender.alive)  # True 打印回调函数状态
print(sys.getrefcount(s1))  # 3
del s1  # 删除{1,2,3}的名字s1
print(ender.alive)  # True 但是此时仍有名字s2指向了 该对象因此对象没有删除
print(sys.getrefcount(s2))  # 2
# 当把名字s2指向其他地方，对象{1,2,3}就没有在使用
# 因此被垃圾回收机制删除。（触发回调函数）
s2 = '11'  # 拜拜了，您内~
print(ender.alive)  # False




