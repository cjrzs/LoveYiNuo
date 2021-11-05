Listvalue = [1,5,6,2,4,3]
ListPointer = [3,2,-1,5,1,4]
head = 0
print(Listvalue[head])
next = ListPointer[head]
while next!=-1:
    print(Listvalue[next])
    next = ListPointer[next]

