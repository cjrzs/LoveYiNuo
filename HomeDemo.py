class Home:
     def  __init__(self,area):
          self.area = area
          self.containsItems = []
          self.light = "on"
     def __str__(self):
          msg = "家的面积为："+str(self.area)
          if len(self.containsItems)>0:
               msg += "\t家里有:"
               for temp in self.containsItems:
                    msg += temp.getName()+","
               msg = msg[:-1]
               if self.light == "on":
                    msg += "\n"+"灯是开着的,所有物品可见"
               else:
                    msg += "\n"+"灯是关着的"
          return msg
     def addItem(self,item):
          needArea = item.getArea()
          if self.area>needArea:
               self.containsItems.append(item)
               self.area -= needArea
     def getName(self):
          return self.name
     def getArea(self):
          return self.area
     def turnoff(self):
          self.light = "off"
          for temp in self.containsItems:
               temp.turnoff()
class Bed:
     def __init__(self,name,area):
          self.name = name
          self.area = area
          self.light = "on"
     def __str__(self):
          msg = self.name+"的面积为："+str(self.area)
          msg += "\n" + "当前物品可见度为："+ self.light
          return msg
     def getName(self):
          return self.name
     def getArea(self):
          return self.area
     def turnoff(self):
          self.light = "off"
#创建一个家的对象
home = Home(1000)
print(home)
#创建一张床
bed = Bed("席梦思",64)
print(bed)
bed2 = Bed("软绵绵",230)
print(bed2)
#将床放到家里面
home.addItem(bed)
home.addItem(bed2)
print(home)
#关灯
home.turnoff()
print(bed)
print(bed2)
