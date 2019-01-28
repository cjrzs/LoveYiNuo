arr1 = [1,5,8,12,16]
arr2 = [2,4,7,9]
ins = 0 #arr2的指针初始化
arr3 = arr1.copy()  #初始化的arr1,以便原数组不变
for i in range(0,len(arr2)):
    while ins < len(arr1):
        if arr2[i] <= arr1[ins]:
            arr3.insert(ins+i,arr2[i])    #将数组2插入到数组3中。
            break
        else:
            ins+=1
    else:
            arr3 = arr3 + arr2[i:]
print(arr3)
