ListValue = [1,5,6,2,7,3]
ListRight = [3,2,4,5,-1,1]
ListLeft = [-1,5,1,0,2,3]
head = ListLeft.index(-1)
print(ListValue[head])
next = ListRight[head]
while next > -1:
    print(ListValue[next])
    next = ListRight[next]
print("/n")

right = 1
left = 2
value = 0
LinkedList = [[1,3,-1],[5,2,5],[6,4,1],[2,5,0],[7,-1,2],[3,1,3]]
head = 0
print(LinkedList[head][value])
next = LinkedList[head][right]
while next > -1:
    print(LinkedList[next][value])
    next = LinkedList[next][right]