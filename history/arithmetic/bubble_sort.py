'''
冒泡排序:第一层循环确定循环次数，第二层循环确定位置。
因为确定的是最最后一个数是什么，所以每次判断都要重头开始判断。
优化：1、添加标志符
     2、双向冒泡
'''
import random
def bubbleSort0(nums):
    #第一层循环确定循环次数，最后一个数不需要参与循环所以要用长度-1
    for i in range(len(nums)-1):
        #第二层循环依次比较大小，确定好的不用比较所以-i
        for j in range(len(nums)-i-1):
            if nums[j] < nums[j+1]:
                nums[j],nums[j+1]=nums[j+1],nums[j]
    return nums

def bubbleSort1(nums):
    for i in range(len(nums)-1):
        flag = False   #假设没有做交换,此时应该退出循环
        for j in range(len(nums)-i-1):
            if nums[j] < nums[j+1]:
                nums[j],nums[j+1]=nums[j+1],nums[j]
                flag = True #做了交换标识符为True，继续与下一个比较
        if not flag:  #没有做交换则break
            break
    return nums

#双向冒泡，因为没有交换的区域就是有序的，所以每次扫描都更新前后的界限。减少扫描范围。
def bubbleSort2(nums):
    low,high = 0,len(nums)-1 #因为要从前后双向排序，所以设定前后边界
    while low<high:  #前边界小于后边界说明还需要继续排序
        flag = low   #flag最后一次发生交换的位置
        # print(flag)
        for j in range(low,high):  #正向扫描，与基本泡泡一样
            if nums[j] < nums[j+1]:
                nums[j],nums[j+1]=nums[j+1],nums[j]
                flag = j  #记录下最后发生交换的位置
                # print(flag)
        high = flag
        # print(flag)
        # print(high)
        # print(low)
        for j in range(high,low,-1):  #逆向扫描，与基本泡泡一样
            if nums[j] > nums[j-1]:
                nums[j], nums[j-1] = nums[j-1], nums[j]
                flag = j
        low = flag
        # print(low)
    return nums
def bubbleSort3(nums):
    for i in range(len(nums)-1):
        for j in range(i+1,len(nums)-1):
            if nums[i]<nums[j]:
                nums[i], nums[j] = nums[j], nums[i]
    return nums
a = list(range(1,10))
random.shuffle(a)
print(a)
print(bubbleSort0(a))
print(bubbleSort1(a))
print(bubbleSort2(a))
print(bubbleSort3(a))
