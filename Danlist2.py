LinkedList = [[1,3],[5,2],[6,-1],[2,5],[4,1],[3,4]]
value = 0
pointer = 1
head = 0
print(LinkedList[head][value])
next = LinkedList[head][pointer]
while next!= -1:
    print(LinkedList[next][value])
    next = LinkedList[next][pointer]