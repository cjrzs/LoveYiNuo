arr = [1,2,3,4,5,6,7,8,9,11,13,16,23,67,87]
head,tail = 0,len(arr)
search = int(input("Enter a number："))
while tail - head > 1:  #当尾指针减去头指针等于一时查找范围内只有head指向的数
    mid  = int((tail - head)/2)   #折半查找
    if arr[mid] < search:         #折半查到的数小于输入的数
        tail = mid                #将折半指针变成尾指针
    elif arr[mid] > search:       #折半查到的数大于输入的数
        head = mid+1              #将折半指针加一变成头指针
    elif arr[mid] == search:
        ans = mid
        break
else:                             #尾指针减去头指针不大于一
    if search == arr[head]:       #头指针对应元素等于查找的数
        ans = head
    else:
        ans = -1                  #头指针对应元素不是要查找的数，该数不在数组内输出-1
print(ans)

