#输入一个目标值，在数组中寻找两个数，这两个数的和等于该值
#使用指针操作
def twosum(nums,target):
    res = []   #一个列表用于保存结果
    nums2 = nums[:]   #深拷贝将原列表复制出一个新列表
    nums2.sort()
    left = 0
    right = len(nums2)-1
    while left<right:
        if nums2[left]+nums2[right] == target:
            for i in range(0,len(nums)):
                if nums2[left] == nums[i]:
                    res.append(i)
                    break
            for i in range(len(nums)-1,-1,-1):
                if nums2[right] == nums[i]:
                    res.append(i)
                    break
            res.sort()
            break
        elif nums2[left]+nums2[right] < target:
            left = left+1
        elif nums2[left]+nums2[right] > target:
            right = right-1
    return res[0]+1,res[1]+1   #返回值是数据在列表中的位置而不是下标，所以将下标+1.
#使用哈希算法进行操作
def twonums2(nums,target):
    dict = {}
    for i in range(len(nums)):
        m = nums[i]
        if target-m in dict:          #判定target-m是不是在字典中
            return (dict[target-m]+1,i+1)
        dict[m] = i                   #记录这对键值

if __name__ == '__main__':
    nums = [3,7,5,10,4]
    print(twosum(nums,11))
    print(twonums2(nums,11))
