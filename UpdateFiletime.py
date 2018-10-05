import os
import datetime
import time

#获取指定路径下所有文件名
path = input("请输入想要转换的目录(后面加'\\')：")
filename = os.listdir(path)
#获取当前时间到分钟转换成整形
date2 = datetime.datetime.now()
datestr = date2.strftime("%Y%m%d%H%M")
dateint = int(datestr)
#循环，依次进行重命名
for name in filename:
     #获取当前时间并转换成字符串
#     date = datetime.datetime.now()
     dateint = dateint+3
     print(dateint)
     os.rename(path+name,path+str(dateint)+"00"+".jpg")
#     time.sleep(1)

#以当前时间往后三分钟命名








