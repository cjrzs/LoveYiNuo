import requests
import os
import re
url = "http://tu.duowan.com/index.php?r=show/getByGallery/&gid=137780&_=1540544402109"
response = requests.get(url)
data = response.json()  #将json格式得数据转换成字典

dir_name = data['gallery_title'] #套图文件夹名
dir_name = re.sub(r'[\\]/*|<>:','',dir_name)#规范文件夹名称。re.sub（）将第一个参数替换成中间的参数，替换的是最后一个参数中的内容
#创建这个文件夹
if not os.path.exists(dir_name): #判断是否存在这个 名字的文件夹
    os.makedirs(dir_name) #创建
#循环下载图片
for index,item in enumerate(data['picInfo']):    #enumerate()循环其中的每一条数据
    img_name = '{}.{}.{}'.format(
        index,   #图片序号
        re.sub(r'[\\*|<>:/"]', '', item['cmt_md5']),
        item['source'].split('.')[-1]  #拿到后缀
    )
    print(item)
    res = requests.get(item['source']) #下载图片
    #写到文件中
    with open(os.path.join(dir_name,img_name),'wb') as f:   #join是拼接得意思将文件名和他的目录拼接再一起
        f.write(res.content)  #content是二进制格式，因为下载的图片所以得用它

